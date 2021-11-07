OWNER="../../wallet-owner.pem"
ADDRESS=$(erdpy data load --key=address-devnet)
PROXY="https://devnet-gateway.elrond.com"
CHAIN_ID="D"
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction-devnet)
SC_ADDRESS="erd1qqqqqqqqqqqqqpgqx7yu353x2k2lckn062dcrndschk805manqjsfp0a0w"
EGLD="1000000000000000000" # 18 decimal


deploy() {
    the_nft_price="500000000000000000"

    erdpy --verbose contract deploy --project=${PROJECT} --recall-nonce \
        --pem=${OWNER} \
        --gas-limit=599000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --arguments $the_nft_price \
        --outfile="deploy-devnet.interaction.json" --send || return

    TRANSACTION=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emitted_tx']['address']")

    erdpy data store --key=address-devnet --value=${ADDRESS}
    erdpy data store --key=deployTransaction-devnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

issuePresaleToken() {
    presale_token_name="0x$(echo -n 'GuardianPresale' | xxd -p -u | tr -d '\n')"
    presale_token_ticker="0x$(echo -n 'GUARDIANP' | xxd -p -u | tr -d '\n')"

    erdpy --verbose contract call ${SC_ADDRESS} --recall-nonce \
        --pem=${OWNER} \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --gas-limit=60000000 \
        --value=50000000000000000 \
        --function="issuePresaleToken" \
        --arguments $presale_token_name $presale_token_ticker \
        --send || return
}

setLocalRolesPresaleToken() {
    erdpy --verbose contract call ${SC_ADDRESS} --recall-nonce \
          --pem=${OWNER} \
          --proxy=${PROXY} --chain=${CHAIN_ID} \
          --gas-limit=60000000 \
          --function=setLocalRolesPresaleToken \
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