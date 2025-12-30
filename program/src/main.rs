// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use anyhow::Result;
use sp1_zkvm::io::commit;
use std::collections::{HashMap, HashSet};
use zktls_att_verification::attestation_data::verify_attestation_data;
use zktls_att_verification::attestation_data::AttestationConfig;

mod errors;
use errors::{ZkErrorCode, ZktlsError};
mod structs;
use structs::{AttestationMetaStruct, PublicValuesStruct};

const SPOT_BALANCE_URL: &str = "https://api.binance.com/api/v3/account";
const SPOT_URL: &[&str] = &[SPOT_BALANCE_URL];
const STABLE_COINS: &[&str] = &[
    "USDT", "USDC", "FDUSD", "TUSD", "USDE", "XUSD", "USD1", "BFUSD", "USDP", "DAI", "USDF",
];

fn app_spot(
    pv: &mut AttestationMetaStruct,
    attestation_data: &String,
    attestation_config: &AttestationConfig,
    asset_bals: &mut HashMap<String, f64>,
) -> Result<(), ZktlsError> {
    //
    // 1. Verify
    let mut attestation_config = attestation_config.clone();
    attestation_config.url = SPOT_URL.iter().map(|s| s.to_string()).collect();
    let attestation_config = serde_json::to_string(&attestation_config).unwrap();
    let (attestation_data, _, messages) = verify_attestation_data(&attestation_data, &attestation_config)
        .map_err(|e| zkerr!(ZkErrorCode::VerifyAttestation, e.to_string()))?;

    pv.task_id = attestation_data.public_data[0].taskId.clone();
    pv.report_tx_hash = attestation_data.public_data[0].reportTxHash.clone();
    pv.attestor = attestation_data.public_data[0].attestor.clone();
    pv.base_urls.push(SPOT_BALANCE_URL.to_string());

    //
    // 2. Do some valid checks
    // In the vast majority of cases, it is legal. Data is extracted while the inspection is conducted.
    let msg_len = messages.len();
    let requests = attestation_data.public_data[0].attestation.request.clone();
    let requests_len = requests.len();
    ensure_zk!(requests_len == msg_len, zkerr!(ZkErrorCode::InvalidMessagesLength));

    let mut i = 0;
    let mut uid_paths = vec![];
    uid_paths.push("$.uid");

    let mut bal_paths = vec![];
    bal_paths.push("$.balances[*].asset");
    bal_paths.push("$.balances[*].free");
    bal_paths.push("$.balances[*].locked");

    pv.timestamp = u128::MAX;
    let mut uids = vec![];
    for request in requests {
        let ts = request
            .url
            .split("timestamp=")
            .nth(1)
            .and_then(|s| s.split('&').next())
            .filter(|s| !s.is_empty())
            .ok_or(zkerr!(ZkErrorCode::CannotFoundTimestamp))?
            .parse::<u128>()
            .map_err(|_| zkerr!(ZkErrorCode::ParseTimestampFailed))?;
        pv.timestamp = pv.timestamp.min(ts);

        // check url
        if !request.url.starts_with(SPOT_BALANCE_URL) {
            return Err(zkerr!(ZkErrorCode::InvalidRequestUrl));
        }

        {
            // uid
            let json_value = messages[i]
                .get_json_values(&uid_paths)
                .map_err(|e| zkerr!(ZkErrorCode::GetJsonValueFail, e.to_string()))?;

            ensure_zk!(json_value.len() == 1, zkerr!(ZkErrorCode::InvalidJsonValueSize));

            let uid = json_value[0].trim_matches('"').to_string();
            uids.push(uid);
        }

        {
            // balance
            let json_value = messages[i]
                .get_json_values(&bal_paths)
                .map_err(|e| zkerr!(ZkErrorCode::GetJsonValueFail, e.to_string()))?;

            ensure_zk!(
                json_value.len() % bal_paths.len() == 0,
                zkerr!(ZkErrorCode::InvalidJsonValueSize)
            );

            let size = json_value.len() / bal_paths.len();
            for j in 0..size {
                let asset = json_value[j].trim_matches('"').to_ascii_uppercase();
                let free: f64 = json_value[size + j].trim_matches('"').parse().unwrap_or(0.0);
                let locked: f64 = json_value[size * 2 + j].trim_matches('"').parse().unwrap_or(0.0);
                *asset_bals.entry(asset.to_string()).or_insert(0.0) += free + locked;
            }
        }

        i += 1;
    }

    // Is the account duplicate?
    let mut seen = HashSet::new();
    ensure_zk!(
        !uids.iter().any(|x| !seen.insert(x)),
        zkerr!(ZkErrorCode::DuplicateAccount)
    );

    Ok(())
}
fn app_main(pv: &mut PublicValuesStruct) -> Result<(), ZktlsError> {
    let config_data: String = sp1_zkvm::io::read();
    let attestations: Vec<String> = sp1_zkvm::io::read(); // 0:Spot; 1:Spot
    ensure_zk!(attestations.len() == 2, zkerr!(ZkErrorCode::InvalidAttestationLength));

    let attestation_config: AttestationConfig =
        serde_json::from_str(&config_data).map_err(|e| zkerr!(ZkErrorCode::ParseConfigData, e.to_string()))?;
    let mut asset_bals: HashMap<String, f64> = HashMap::new();

    // Spot
    let mut spot_am = AttestationMetaStruct::default();
    app_spot(&mut spot_am, &attestations[0], &attestation_config, &mut asset_bals)?;
    pv.attestation_meta.push(spot_am);

    // Spot
    let mut spot_am = AttestationMetaStruct::default();
    app_spot(&mut spot_am, &attestations[1], &attestation_config, &mut asset_bals)?;
    pv.attestation_meta.push(spot_am);

    // Summary assets by Category
    let mut binance_asset: HashMap<String, f64> = HashMap::new();
    let mut stablecoin_sum = 0.0;
    for (k, v) in asset_bals {
        if STABLE_COINS.contains(&k.as_str()) {
            stablecoin_sum += v;
        } else {
            binance_asset.insert(k, v);
        }
    }
    binance_asset.insert("STABLECOIN".to_string(), stablecoin_sum);
    pv.asset_balance.insert("binance".to_string(), binance_asset);

    Ok(())
}

pub fn main() {
    let mut pv = PublicValuesStruct::default();
    pv.kind = "asset-balance".to_string();
    pv.version = "0.1.0".to_string();
    pv.project_id = "2001569168033316864".to_string();
    if let Err(e) = app_main(&mut pv) {
        println!("Error: {} {}", e.icode(), e.msg());
        pv.status = e.icode();
    } else {
        println!("OK");
    }
    commit(&pv);
}
