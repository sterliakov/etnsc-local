ARG IMAGE_REF=latest
FROM sterliakov/electroneum:$IMAGE_REF

COPY --chmod=500 entrypoint.sh /
ENTRYPOINT [ "/entrypoint.sh" ]
