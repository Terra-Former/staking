use cosmwasm_std::MessageInfo;
use cosmwasm_std::testing::mock_info;

pub const DEFAULT_SENDER: &str = "DefaultSender";
pub const CONTRACT_CREATOR: &str = "ContractCreator";
pub const TOKEN: &str = "Token";
pub const TERRASWAP_ROUTER: &str = "TerraswapRouter";

pub fn default_sender() -> MessageInfo {
    mock_info(DEFAULT_SENDER, &[])
}

pub fn contract_creator() -> MessageInfo {
    mock_info(CONTRACT_CREATOR, &[])
}

pub fn token() -> MessageInfo {
    mock_info(TOKEN, &[])
}
