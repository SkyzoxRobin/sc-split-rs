OWNER="../../wallet-owner.pem"
ADDRESS=$(erdpy data load --key=address-devnet)
PROXY="https://devnet-gateway.elrond.com"
CHAIN_ID="D"
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction-devnet)
SC_ADDRESS="erd1qqqqqqqqqqqqqpgqe4mzxtternw66skxllwelaf6uwt6gx5e2yusxgfpn9"
EGLD="1000000000000000000" # 18 decimal


deploy() {
    erdpy --verbose contract deploy --project=${PROJECT} --recall-nonce \
        --pem=${OWNER} \
        --gas-price=1499999999 \
        --gas-limit=1499999999 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
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
        --pem=${OWNER} \
        --gas-limit=20000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="splitEGLD" \
        --arguments $recipients $amount 1 \
        --send || return
}
# $1 = esdt token amount - amount_esdt
# $recipient_2 $amount_to_recipient2
splitESDT() {
    token_id="0x$(echo -n 'AEGLD-6e6df3' | xxd -p -u | tr -d '\n')"
    method_name="0x$(echo -n 'splitESDT' | xxd -p -u | tr -d '\n')"
    recipient_1="0x$(erdpy wallet bech32 --decode 'erd1vcastmazp4w40fn92pztrw606pmqqtg8wgjprncfds6h9ryxdmqspz5v4v')"
    amount_to_recipient1="0x$(printf '%x' 1000000000000000000)"
    amount_esdt="0x$(printf '%x' 2000000000000000000)"
    recipient_2="0x$(erdpy wallet bech32 --decode 'erd17yva92k3twysqdf4xfw3w0q8fun2z3ltpnkqldj59297mqp9nqjs9qvkwn')"
    amount_to_recipient2="0x$(printf '%x' 1000000000000000000)"

    erdpy --verbose contract call ${SC_ADDRESS} --recall-nonce \
        --pem=${OWNER} \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --gas-limit=1000000000 \
        --function=ESDTTransfer \
        --arguments $token_id $amount_esdt $method_name $recipient_1 $amount_to_recipient1 $recipient_2 $amount_to_recipient2 \
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