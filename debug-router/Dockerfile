ARG TAG=v1.61.0

FROM ghcr.io/apollographql/router:${TAG}

USER root
RUN mkdir -p /opt/scripts
COPY router.yml /opt
COPY main.rhai /opt/scripts

USER router
ENTRYPOINT ["/dist/router_wrapper.sh", "--config=/opt/router.yml", "--hot-reload"]
