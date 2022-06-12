import Array "mo:base/Array";
import Debug "mo:base/Debug";
import Error "mo:base/Error";
import Nat64 "mo:base/Nat64";
import Principal "mo:base/Principal";
import Result "mo:base/Result";
import TrieSet "mo:base/TrieSet";

import Common "canister:common";
import Types "Types";
import Utils "Utils";
import Oracle "Oracle";

actor class Self(payload : Types.InitPayload) {

    // Actor definition to handle interactions with the BTC canister.
    type BTC = actor {
        // Gets the balance from the BTC canister.
        get_balance : Types.GetBalanceRequest -> async Types.GetBalanceResponse;
        // Retrieves the UTXOs from the BTC canister.
        get_utxos : Types.GetUtxosRequest -> async Types.GetUtxosResponse;
        // Sends a transaction to the BTC canister.
        send_transaction : (Types.SendTransactionRequest) -> async Types.SendTransactionResponse;
    };

    // The canister's private key in "Wallet Import Format".  
    let PRIVATE_KEY_WIF : Text = "L2C1QgyKqNgfV7BpEPAm6PVn2xW8zpXq6MojSbWdH18nGQF2wGsT";
    // Used to interact with the BTC canister.
    let btc : BTC = actor(Principal.toText(payload.bitcoin_canister_id));
    // Stores outpoints the have been spent.
    let spent_outpoints : Utils.OutPointSet = Utils.OutPointSet();

    public func get_btc_price() : async Types.GetBtcPriceResponse {
        await Oracle.get_btc_price()
    }
};
