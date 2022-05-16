FROM ghcr.io/cosmos/gaia:sha-fca0a63

COPY ./test.sh .

ENTRYPOINT [ "./test.sh" ]