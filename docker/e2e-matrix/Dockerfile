FROM mcr.microsoft.com/dotnet/sdk:10.0
RUN apt-get update \
  && apt-get install -y --no-install-recommends curl ca-certificates gnupg build-essential python3 \
  && curl -fsSL https://deb.nodesource.com/setup_22.x | bash - \
  && apt-get install -y --no-install-recommends nodejs \
  && rm -rf /var/lib/apt/lists/*
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /work
CMD ["node", "scripts/e2e-matrix.mjs"]
