FROM docker.io/debian@sha256:ce3f6651cf74f6c97017be51ca0dee1424a7d65caad5df54e6fb961344593343

# self_dev
COPY ./self_dev/syscaller/target/release/syscaller /tools/sys
COPY ./self_dev/enc_executor/target/release/enc_executor /tools/enc
COPY ./self_dev/speed_executor/target/release/speed_executor /tools/speed

# oss
COPY ./oss/net/release/l2 tools/l2
COPY ./oss/net/release/udp /tools/udp

# install neccessary software as well
#RUN apt update && 
#    apt install -y 
#            net-tools 
#            iproute2

ENTRYPOINT ["tail", "-f", "/dev/null"]
