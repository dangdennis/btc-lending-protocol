import Types "Types";

module Oracle {
    // Retrieves the BTC/USD price in cents.
    //
    // Todo:
    // 1. Attach to a real oracle.
    public func get_btc_price() : async Types.GetBtcPriceResponse {
        #Ok(2857379)
    };
}