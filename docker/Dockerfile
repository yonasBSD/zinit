#trivy:ignore:DS001
FROM cgr.dev/chainguard/wolfi-base:latest

USER nobody

RUN mkdir -p /tmp/zinit
COPY zinit /sbin/zinit

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
  CMD zinit list || exit 1
ENTRYPOINT ["/sbin/zinit", "init"]
