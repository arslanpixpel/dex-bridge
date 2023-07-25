-- Mapping of tokens 
-- CREATE TABLE IF NOT EXISTS token_maps (
--        id SERIAL8 PRIMARY KEY UNIQUE,
--        -- The address of a root token on Ethereum.
--        root BYTEA NOT NULL,
--        -- Contract address of the mapped token on Concordium.
--        child_index INT8 NOT NULL,
--        child_subindex INT8 NOT NULL,
--        -- Name of the token on Ethereum.
--        eth_name TEXT NOT NULL,
--        -- The number of decimals of the token.
--        decimals SMALLINT NOT NULL,
--        CONSTRAINT token_maps_root_unique UNIQUE (root)
--        );
INSERT INTO token_maps(root, child_index, child_subindex, eth_name, decimals)
values(decode('43D8814FdFB9B8854422Df13F1c66e34E4fa91fD', 'hex'), 4604, 0, 'USDC', 6);

INSERT INTO token_maps(root, child_index, child_subindex, eth_name, decimals)
values(decode('EeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE', 'hex'), 4603, 0, 'KAVA', 18);