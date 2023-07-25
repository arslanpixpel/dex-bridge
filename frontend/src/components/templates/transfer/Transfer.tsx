{
    /* eslint-disable @next/next/no-img-element */
    /* Disabling this, as we cannot ensure the src of the icon, which this specific version of nextJS requires*/
}
import Button from "@components/atoms/button/Button";
import Input from "@components/atoms/input/input";
import PageWrapper from "@components/atoms/page-wrapper/PageWrapper";
import { CornucopiaLogo } from "@components/molecules/logo/Logo";
import useCCDWallet from "@hooks/use-ccd-wallet";
import useEthWallet from "@hooks/use-eth-wallet";
import { useAsyncMemo } from "@hooks/utils";
import Image from "next/image";
import Link from "next/link";
import { useRouter } from "next/router";
import { FC, useCallback, useContext, useEffect, useMemo, useState } from "react";
import { TokenWithIcon, useTokens } from "src/api-query/queries";
import { Components } from "src/api-query/__generated__/AxiosClient";
import { BridgeDirection, routes } from "src/constants/routes";
import useCCDContract from "src/contracts/use-ccd-contract";
import useGenerateContract from "src/contracts/use-generate-contract";
import { noOp } from "src/helpers/basic";
import parseWallet from "src/helpers/parseWallet";
import { appContext } from "src/root/app-context";
import { useTransactionFlowStore } from "src/store/transaction-flow";
import { QueryRouter } from "src/types/config";
import ArrowDownIcon from "../../../../public/icons/arrow-down-icon.svg";
import ConcordiumIcon from "../../../../public/icons/concordium-icon.svg";
import EthereumIcon from "../../../../public/icons/ethereum-icon.svg";
import SwapIcon from "../../../../public/icons/sort-icon.svg";
import Text from "../../atoms/text/text";
import {
    Coin,
    CoinContainer,
    CoinPicker,
    CoinSelect,
    Dropdown,
    DropdownButton,
    DropdownList,
    FirstRow,
    LinkWrapper,
    MaxGapRow,
    OrderText,
    SecondRow,
    StyledCoinText,
    StyledContainer,
    StyledWalletDisplay,
    SwapLink,
} from "./Transfer.style";
import { ethers } from "ethers";
import network from "@config/network";
import { CCD_MAINNET_GENESIS, CCD_TESTNET_GENESIS } from "src/constants/network";
import { toFractionalAmount } from "src/helpers/number";

interface ChainType {
    id: number;
    name: string;
    icon: string;
    account?: string | null;
    connect: (() => void) | null;
}

interface ChainBoxProps {
    chain: ChainType;
    text: string;
}

const ChainBox: React.FC<ChainBoxProps> = ({ chain, text }) => {
    // state for the copied address
    const [copied, setCopied] = useState<boolean>(false);

    const walletCopyHandler = () => {
        setCopied(true);
        navigator.clipboard.writeText(chain.account || "");
    };

    useEffect(() => {
        const copyTimeout = setTimeout(() => {
            setCopied(false);
        }, 1000);

        return () => clearTimeout(copyTimeout);
    }, [copied]);

    return (
        <CoinContainer>
            <OrderText>{text}</OrderText>
            <CoinSelect>
                <CoinPicker>
                    <Coin>
                        <Image src={chain.icon} alt={`${chain.name} icon`} height="23" width="23" />
                        <StyledCoinText fontWeight="light">{chain.name}</StyledCoinText>
                    </Coin>
                    {chain.account ? (
                        <StyledWalletDisplay copied={copied} onClick={walletCopyHandler}>
                            {parseWallet(chain.account)}
                        </StyledWalletDisplay>
                    ) : (
                        <Button
                            variant="connect"
                            onClick={() => {
                                chain.connect && chain.connect();
                            }}
                        >
                            Connect
                        </Button>
                    )}
                </CoinPicker>
            </CoinSelect>
        </CoinContainer>
    );
};

type SelectTokenProps = {
    isDeposit: boolean;
    tokens: TokenWithIcon[] | undefined;
    onSelect(token: Components.Schemas.TokenMapItem): void;
    onMax(decimalAmount: string): void;
    selected: Components.Schemas.TokenMapItem | undefined;
    /** As a fractional value */
    selectedTokenBalance: string | undefined;
    /** As a fractional value. Falls back to `selectedTokenBalance` */
    maxTransferValue?: string;
};

const SelectToken: FC<SelectTokenProps> = ({
    tokens,
    onSelect,
    onMax,
    selected,
    isDeposit,
    selectedTokenBalance,
    maxTransferValue,
}) => {
    const [open, setOpen] = useState(false);
    const toggle = () => setOpen((o) => !o);
    const fallbackIconUrl: string = isDeposit ? EthereumIcon.src : ConcordiumIcon.src;
    const selectedIconUrl = selected
        ? tokens?.find(({ token }) => token.eth_address === selected?.eth_address)?.iconUrl ?? fallbackIconUrl
        : undefined;

    const sortedTokens = tokens?.slice().sort(({ token: a }, { token: b }) => {
        const isLess = isDeposit ? a.eth_name < b.eth_name : a.ccd_name < b.ccd_name;
        return isLess ? -1 : 1;
    });

    return (
        <MaxGapRow>
            <DropdownButton onClick={toggle}>
                {selected && selectedIconUrl && (
                    <img
                        src={selectedIconUrl}
                        alt={`${isDeposit ? selected.eth_name : selected.ccd_name} icon`}
                        height="23.13"
                        width="23.13"
                    />
                )}
                <Text fontWeight="light" style={{ paddingLeft: selectedIconUrl ? 7 : undefined }}>
                    {selected ? (isDeposit ? selected.eth_name : selected.ccd_name) : "Select Token"}
                </Text>
                <Dropdown>
                    <Image src={ArrowDownIcon.src} alt="dropdown icon" height="12" width="12" />
                </Dropdown>
            </DropdownButton>
            <Button variant="max" onClick={() => onMax(maxTransferValue ?? selectedTokenBalance ?? "")}>
                <Text fontSize="10" fontWeight="light">
                    Max
                </Text>
            </Button>
            <DropdownList open={open}>
                {sortedTokens?.map((tokenData) => {
                    const {
                        token: { ccd_name, ccd_contract, eth_name, eth_address },
                        iconUrl,
                    } = tokenData;

                    return (
                        <Coin
                            onClick={() => {
                                onSelect(tokenData.token);
                                setOpen(false);
                            }}
                            key={
                                isDeposit
                                    ? `${eth_name + eth_address}`
                                    : `${ccd_name + ccd_contract?.index + ccd_contract?.subindex}`
                            }
                        >
                            <img
                                src={iconUrl ?? fallbackIconUrl}
                                alt={`${isDeposit ? eth_name : ccd_name} icon`}
                                height="23.13"
                                width="23.13"
                            />
                            <StyledCoinText fontWeight="light">{isDeposit ? eth_name : ccd_name}</StyledCoinText>
                        </Coin>
                    );
                })}
            </DropdownList>
        </MaxGapRow>
    );
};

type TransferRouteQuery = {
    reset?: boolean;
};

export type MaxTransferValue = {
    value: bigint;
    deductedFee: bigint;
};

interface Props {
    isDeposit?: boolean;
    /** If the only limit to this is the available balance in the wallet, this can be left undefined */
    maxTransferValue?: MaxTransferValue;
}

const Transfer: React.FC<Props> = ({ isDeposit = false, maxTransferValue }) => {
    const tokensQuery = useTokens();
    const {
        query: { reset = false },
        isReady,
        prefetch,
        replace,
        pathname,
    } = useRouter() as QueryRouter<TransferRouteQuery>;
    const { context, connect, hasWallet } = useEthWallet();
    const { ccdContext, connectCCD: connectCcd, hasWallet: hasCcdWallet } = useCCDWallet();
    const { push } = useRouter();
    const { isTablet } = useContext(appContext);
    const { token, amount = 0n, setToken, setAmount, clear: clearTransactionFlow } = useTransactionFlowStore();

    // Keeps track of whether "continue" has been pressed. Used to not show validation error message prematurely.
    const [submitted, setSubmitted] = useState(false);

    // state dependent hooks
    const { getBalance: getEthTokenBalance } = useGenerateContract(
        token?.eth_address || "", // address or empty string because the address is undefined on first renders
        !!token // plus it's disabled on the first render anyway
    );
    const { balanceOf: getCcdTokenBalance } = useCCDContract(ccdContext.account, ccdContext.isActive);

    const isLoggedIn = !!context?.account && !!ccdContext.account;
    const transferButtonDisabled = !isLoggedIn || !token;
    const nextRoute = useMemo(() => (isDeposit ? routes.deposit.overview : routes.withdraw.overview), [isDeposit]);
    const swapRoute = useMemo(() => (isDeposit ? routes.withdraw.path : routes.deposit.path), [isDeposit]);
    const toTokenIntegerAmount = useCallback(
        (decimalAmount: string) => {
            if (token === undefined) {
                throw new Error("Token expected to be available");
            }

            return ethers.utils.parseUnits(decimalAmount, token.decimals).toBigInt();
        },
        [token]
    );
    const toTokenDecimalAmount = useCallback(
        (amount: bigint) => {
            if (token === undefined) {
                throw new Error("Token expected to be available");
            }

            return toFractionalAmount(amount, token.decimals);
        },
        [token]
    );
    const [inputAmount, setInputAmount] = useState(
        token !== undefined && amount !== undefined ? toTokenDecimalAmount(amount) ?? "0" : "0"
    );

    // tokens available in the dropdown
    const tokens = useMemo(() => {
        if (tokensQuery.status !== "success") {
            return undefined;
        }
        return tokensQuery.data;
    }, [tokensQuery]);

    const tokenBalance = useAsyncMemo(
        async () => {
            if (!isLoggedIn || !token) {
                return undefined;
            }

            return isDeposit ? getEthTokenBalance() : getCcdTokenBalance(token);
        },
        noOp,
        [isLoggedIn, token, getCcdTokenBalance, getEthTokenBalance]
    );
    const decimalTokenBalance = useMemo(() => {
        if (tokenBalance === undefined || token === undefined) {
            return undefined;
        }

        return toTokenDecimalAmount(tokenBalance);
    }, [tokenBalance, token, toTokenDecimalAmount]);

    const minTransferValue = useMemo(() => {
        if (token?.decimals === undefined) {
            return undefined;
        }

        return 1 / 10 ** token.decimals;
    }, [token?.decimals]);

    const connectEth = async () => {
        // if (!hasWallet) {
        //     window.alert("MetaMask could not be found.");
        //     return;
        // }

        return connect();
    };

    const connectCcdHandleNetwork = async () => {
        if (!hasCcdWallet) {
            window.alert("Concordium wallet could not be found.");
            return;
        }

        try {
            await connectCcd();
        } catch {
            if (network.ccd.genesisHash === CCD_MAINNET_GENESIS) {
                window.alert('Please connect to the "Concordium Mainnet" network in your Concordium wallet');
            } else if (network.ccd.genesisHash === CCD_TESTNET_GENESIS) {
                window.alert('Please connect to the "Concordium Testnet" network in your Concordium wallet');
            } else {
                window.alert("Please connect to the correct network in your Concordium wallet");
            }
        }
    };

    const chains: ChainType[] = [
        {
            id: 1,
            name: "Kava",
            account: context.account,
            icon: EthereumIcon.src,
            connect: connectEth,
        },
        {
            id: 2,
            name: "Concordium",
            icon: ConcordiumIcon.src,
            account: ccdContext.account,
            connect: connectCcdHandleNetwork,
        },
    ];

    const inputValidation = useMemo(() => {
        if (token === undefined || tokenBalance === undefined) {
            return true;
        }

        try {
            const nAmount = toTokenIntegerAmount(inputAmount) ?? 0n;

            if (nAmount <= 0n) {
                return "Value has to be above 0";
            }

            return nAmount <= tokenBalance || "Insufficient funds on account";
        } catch {
            return "Invalid amount";
        }
    }, [inputAmount, tokenBalance, token, toTokenIntegerAmount]);

    const showValidationError = inputValidation !== true && submitted;
    const showEthBalanceWarning = useMemo(() => {
        if (inputValidation !== true) {
            return false;
        }

        try {
            return (
                maxTransferValue !== undefined &&
                ethers.utils.parseEther(inputAmount).toBigInt() > maxTransferValue.value
            );
        } catch {
            return false;
        }
    }, [inputAmount, inputValidation, maxTransferValue]);

    useEffect(() => {
        if (reset && isReady) {
            clearTransactionFlow();
            replace(pathname);
        }
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [reset, isReady]);

    useEffect(() => {
        prefetch(routes.history());
    }, [prefetch]);

    const submitHandler = useCallback(() => {
        setSubmitted(true);

        if (inputValidation !== true || token === undefined) {
            // Abort.
            return;
        }

        const tokenAmount = toTokenIntegerAmount(inputAmount);
        if (tokenAmount === undefined) {
            throw new Error("Could not convert input to token amount");
        }

        setAmount(tokenAmount);
        push({ pathname: nextRoute });
    }, [inputValidation, token, toTokenIntegerAmount, inputAmount, setAmount, push, nextRoute]);

    return (
        <PageWrapper>
            <StyledContainer>
                {isTablet && <CornucopiaLogo isTablet={isTablet} />}
                <Text fontFamily="Roboto" fontSize="24" fontWeight="light" fontColor="TitleText" fontLetterSpacing="0">
                    Transfer
                </Text>
                <FirstRow>
                    {chains
                        .sort((a, b) => (isDeposit ? +a.id - +b.id : +b.id - +a.id))
                        .map((chain, index) => (
                            <ChainBox key={chain.id} chain={chain} text={index === 0 ? "From" : "To"} />
                        ))}
                    <Link href={swapRoute} passHref legacyBehavior>
                        <SwapLink>
                            <Image src={SwapIcon.src} alt="swap icon" width="14" height="11" />
                        </SwapLink>
                    </Link>
                </FirstRow>
                <SecondRow>
                    <SelectToken
                        tokens={tokens}
                        selected={token}
                        isDeposit={isDeposit}
                        onSelect={setToken}
                        onMax={setInputAmount}
                        selectedTokenBalance={decimalTokenBalance}
                        maxTransferValue={
                            maxTransferValue !== undefined
                                ? ethers.utils.formatEther(maxTransferValue.value)
                                : undefined
                        }
                    />
                    <MaxGapRow input>
                        <Input
                            value={inputAmount}
                            onChange={(e) => setInputAmount(e.target.value)}
                            type="number"
                            formNoValidate
                            min={minTransferValue}
                            max={decimalTokenBalance}
                            step={minTransferValue}
                            valid={!showValidationError}
                        />
                        <Text
                            style={{ alignSelf: "flex-end", position: "absolute", right: 0, bottom: -10 }}
                            fontColor="Balance"
                            fontSize="10"
                        >
                            {isLoggedIn && token && decimalTokenBalance ? `Balance: ${decimalTokenBalance}` : " "}
                        </Text>
                    </MaxGapRow>
                    {showValidationError && (
                        <Text fontSize="11" fontColor="Red" style={{ position: "absolute", bottom: "-20px" }}>
                            {inputValidation}
                        </Text>
                    )}
                    {!showValidationError && showEthBalanceWarning && maxTransferValue !== undefined && (
                        <Text fontSize="11" fontColor="Black" style={{ position: "absolute", bottom: "-20px" }}>
                            Exceeds balance + estimated fees ({ethers.utils.formatEther(maxTransferValue.deductedFee)}{" "}
                            ETH)
                        </Text>
                    )}
                </SecondRow>
                <Button variant="primary" disabled={transferButtonDisabled} onClick={submitHandler}>
                    <div style={{ position: "relative" }}>
                        <Text fontSize="16" fontColor={transferButtonDisabled ? "White" : "Brown"} fontWeight="regular">
                            {isDeposit ? "Deposit" : "Withdraw"}
                        </Text>
                    </div>
                </Button>
            </StyledContainer>
            <Link
                href={routes.history(isDeposit ? BridgeDirection.Deposit : BridgeDirection.Withdraw)}
                passHref
                legacyBehavior
            >
                <LinkWrapper hidden={!context.account}>
                    <Text fontSize="12" fontFamily="Roboto" fontColor="Brown">
                        Transaction History
                    </Text>
                </LinkWrapper>
            </Link>
        </PageWrapper>
    );
};

export default Transfer;
