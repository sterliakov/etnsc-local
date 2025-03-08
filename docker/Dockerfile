ARG IMAGE_REF=latest
FROM sterliakov/electroneum:$IMAGE_REF

COPY --chmod=500 entrypoint.sh /
# hadolint ignore=DL3018
RUN apk add --no-cache curl
ENTRYPOINT [ "/entrypoint.sh" ]
