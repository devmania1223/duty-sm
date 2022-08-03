#[test]
fn auction_end_deadline_go() {
    elrond_wasm_debug::mandos_go("mandos/auction_end_deadline.scen.json");
}

#[test]
fn auction_end_max_bid_go() {
    elrond_wasm_debug::mandos_go("mandos/auction_end_max_bid.scen.json");
}

#[test]
fn auction_sell_all_end_deadline_go() {
    elrond_wasm_debug::mandos_go("mandos/auction_sell_all_end_deadline.scen.json");
}

#[test]
fn auction_sell_one_by_one_end_deadline_go() {
    elrond_wasm_debug::mandos_go("mandos/auction_sell_one_by_one_end_deadline.scen.json");
}

#[test]
fn auction_nft_sell_all_go() {
    elrond_wasm_debug::mandos_go("mandos/auction_nft_sell_all.scen.json");
}

#[test]
fn auction_nft_sell_one_by_one_go() {
    elrond_wasm_debug::mandos_go("mandos/auction_nft_sell_one_by_one.scen.json");
}

#[test]
fn create_auction_go() {
    elrond_wasm_debug::mandos_go("mandos/create_auction.scen.json");
}

#[test]
fn auction_with_start_time_go() {
    elrond_wasm_debug::mandos_go("mandos/auction_with_start_time.scen.json");
}

#[test]
fn bid_first_go() {
    elrond_wasm_debug::mandos_go("mandos/bid_first.scen.json");
}

#[test]
fn bid_max_go() {
    elrond_wasm_debug::mandos_go("mandos/bid_max.scen.json");
}

#[test]
fn bid_second_go() {
    elrond_wasm_debug::mandos_go("mandos/bid_second.scen.json");
}

#[test]
fn bid_nft_sell_all_first_go() {
    elrond_wasm_debug::mandos_go("mandos/bid_nft_sell_all_first.scen.json");
}

#[test]
fn bid_nft_sell_all_second_go() {
    elrond_wasm_debug::mandos_go("mandos/bid_nft_sell_all_second.scen.json");
}

#[test]
fn buy_nft_sell_one_by_one_go() {
    elrond_wasm_debug::mandos_go("mandos/buy_nft_sell_one_by_one.scen.json");
}

#[test]
fn buy_nft_sell_one_by_one_second_go() {
    elrond_wasm_debug::mandos_go("mandos/buy_nft_sell_one_by_one_second.scen.json");
}

#[test]
fn init_go() {
    elrond_wasm_debug::mandos_go("mandos/init.scen.json");
}

#[test]
fn invalid_bids_go() {
    elrond_wasm_debug::mandos_go("mandos/invalid_bids.scen.json");
}

#[test]
fn specific_token_auctioned_go() {
    elrond_wasm_debug::mandos_go("mandos/specific_token_auctioned.scen.json");
}

#[test]
fn view_functions_go() {
    elrond_wasm_debug::mandos_go("mandos/view_functions.scen.json");
}

#[test]
fn cancel_auction_go() {
    elrond_wasm_debug::mandos_go("mandos/cancel_auction.scen.json");
}

#[test]
fn cancel_auction_after_end_auction_go() {
    elrond_wasm_debug::mandos_go("mandos/cancel_auction_after_end_auction.scen.json");
}
