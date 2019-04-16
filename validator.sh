if [ ! -f /etc/sawtooth/keys/validator.priv ]; then
    sawadm keygen
fi

if [ -d /var/lib/sawtooth ]; then
    rm /var/lib/sawtooth/*
fi

sawset genesis -k /etc/sawtooth/keys/validator.priv -o init_settings.batch

sawset proposal create \
    -k /etc/sawtooth/keys/validator.priv \
    sawtooth.consensus.algorithm.name=consensus \
    sawtooth.consensus.algorithm.version=0.1 \
    -o consensus.batch

sawadm genesis init_settings.batch consensus.batch

sawtooth-validator -vv \
    --endpoint tcp://127.0.0.1:8800 \
    --bind component:tcp://0.0.0.0:4004 \
    --bind consensus:tcp://0.0.0.0:5005 \
    --bind network:tcp://0.0.0.0:8800 \
    --peering static
