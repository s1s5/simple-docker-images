FROM docker:dind-rootless
ADD https://release.ariga.io/atlas/atlas-linux-amd64-latest /atlas
USER root
RUN chmod +x /atlas
user rootless

ENTRYPOINT ["/atlas"]
