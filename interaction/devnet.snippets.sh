OWNER="wallets/owner/wallet-owner.pem"
ADDRESS=$(erdpy data load --key=address-devnet)
PROXY="https://devnet-gateway.elrond.com"
CHAIN_ID="D"
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction-devnet)
SC_ADDRESS="erd1qqqqqqqqqqqqqpgqp8tsypy7x4m4w4f8n3qn33fjcl2gzkjp2yusq4mlr7"


deploy() {

    erdpy --verbose contract deploy --project=${PROJECT} --recall-nonce \
        --pem=${OWNER} \
        --gas-price=1499999999 \
        --gas-limit=1499999999 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        # --arguments 0 \
        --outfile="deploy-devnet.interaction.json" --send || return

    TRANSACTION=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emitted_tx']['address']")

    erdpy data store --key=address-devnet --value=${ADDRESS}
    erdpy data store --key=deployTransaction-devnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

splitEGLD() {
    #method_name="0x$(echo -n 'splitEGLD' | xxd -p -u | tr -d '\n')"
    recipients="0x$(erdpy wallet bech32 --decode erd14jrw6uyfk9vlv45hjv0rdxxr6um4ccdjk9rwhy75dfwmdpdz2yusr456ry)" 
    amount=""

     erdpy --verbose contract call ${SC_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=20000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="splitEGLD" \
        --arguments $recipients $amount 1 \
        --send || return
}

upgrade() {
    erdpy --verbose contract upgrade ${ADDRESS} --project=${PROJECT} --recall-nonce --pem=${OWNER} --gas-limit=100000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send --outfile="deploy-devnet.interaction.json" || return

    echo ""
    echo "Smart contract upgraded address: ${ADDRESS}"
}

checkDeployment() {
    erdpy tx get --hash=$DEPLOY_TRANSACTION --omit-fields="['data', 'signature']" --proxy=${PROXY}
    erdpy account get --address=$ADDRESS --omit-fields="['code']" --proxy=${PROXY}
}