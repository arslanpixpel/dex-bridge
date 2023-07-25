import Image from "next/image";
import Text from "../../atoms/text/text";
import Hourglass from "../../../../public/icons/Hourglass.svg";
import {
    StyledButtonContainer,
    StyledCircle,
    StyledCircleWrapper,
    StyledHorizontalLine,
    StyledProcessWrapper,
    TransferAmountWrapper,
    ModalTitle,
    Content,
    InfoContainer,
    StyledContainer,
} from "./TransferProgress.style";
import PageWrapper from "@components/atoms/page-wrapper/PageWrapper";
import Button from "@components/atoms/button/Button";
import { useRouter } from "next/router";
import { routes } from "src/constants/routes";
import { useTransactionFlowStore } from "src/store/transaction-flow";
import { Components } from "src/api-query/__generated__/AxiosClient";
import { useMemo, useState } from "react";
import { QueryRouter } from "src/types/config";
import isDeposit from "src/helpers/checkTransaction";
import { useGetTransactionToken } from "@hooks/use-transaction-token";
import { useWalletTransactions } from "src/api-query/queries";
import useEthWallet from "@hooks/use-eth-wallet";
import { toFractionalAmount } from "src/helpers/number";

type Status = {
    message: string;
    isError: boolean;
};

enum TransferStep {
    Added,
    Pending,
    Processed,
    Failed = -1,
}

const transferStepMap: { [p in Components.Schemas.TransactionStatus]: TransferStep } = {
    missing: TransferStep.Added,
    pending: TransferStep.Pending,
    processed: TransferStep.Processed,
    failed: TransferStep.Failed,
};

type BaseProps = {
    transferStatus?: Components.Schemas.TransactionStatus;
    disableContinue?: boolean;
};

type WithdrawProps = BaseProps & {
    isWithdraw: true;
    canWithdraw?: boolean;
    onRequestApproval(
        setError: (message: string) => void,
        setStatus: (message: string | undefined) => void
    ): Promise<void>;
};
type DepositProps = BaseProps & {
    isWithdraw?: false;
};

type Props = WithdrawProps | DepositProps;

const useTransactionDetails = () => {
    const {
        query: { tx },
        isReady,
    } = useRouter() as QueryRouter<{ tx: string }>;

    if (isReady && !tx) throw new Error("Expected transaction hash to be part of url");

    const result = useWalletTransactions();
    const transaction = result.data?.find((t) => {
        const hash = isDeposit(t) ? t.Deposit.origin_tx_hash : t.Withdraw.origin_tx_hash;
        return tx === hash;
    });

    const getToken = useGetTransactionToken();

    const value = { loading: result.isLoading, data: undefined };

    if (transaction === undefined) {
        return value;
    }

    const rawAmount = isDeposit(transaction) ? transaction.Deposit.amount : transaction.Withdraw.amount;
    const tokenQuery = getToken(transaction);

    if (tokenQuery.status !== "success" || tokenQuery.token === undefined) {
        return { loading: value.loading || tokenQuery.status === "loading", data: undefined };
    }

    const token = tokenQuery.token;
    const amount = BigInt(rawAmount);

    const data = {
        amount,
        token,
    };

    return { loading: false, data };
};

export const TransferProgress: React.FC<Props> = (props) => {
    const { transferStatus, isWithdraw = false, disableContinue = false } = props;
    const {
        query: { tx },
        isReady,
        push,
    } = useRouter() as QueryRouter<{ tx: string }>;
    const [status, setStatus] = useState<Status | undefined>();
    const { data: transactionDetails, loading: transactionDetailsLoading } = useTransactionDetails();
    const {
        token: storedToken,
        amount: storedAmount,
        clear: clearFlowStore,
        transactionHash,
    } = useTransactionFlowStore();
    const amount = isReady && tx === transactionHash ? storedAmount : transactionDetails?.amount;
    const token = isReady && tx === transactionHash ? storedToken : transactionDetails?.token;
    const { context, connect: connectEth } = useEthWallet();

    const step = useMemo(() => transferStepMap[transferStatus ?? "missing"], [transferStatus]);
    const decimalAmount = useMemo(() => {
        if (token === undefined || amount === undefined) {
            return undefined;
        }

        return toFractionalAmount(amount, token.decimals);
    }, [amount, token]);

    const setError = (message: string) => setStatus({ isError: true, message });
    const setInfo = (message: string | undefined) =>
        setStatus(message !== undefined ? { isError: false, message } : undefined);

    const continueHandler = async () => {
        if (props.isWithdraw && props.canWithdraw) {
            setStatus(undefined);

            if (!context.active) {
                await connectEth();
            }

            await props.onRequestApproval(setError, setInfo);
        } else {
            push({ pathname: isWithdraw ? routes.withdraw.path : routes.deposit.path, query: { reset: true } });
            clearFlowStore();
        }
    };

    return (
        <PageWrapper>
            <StyledContainer>
                <ModalTitle>
                    <Text
                        fontFamily="Roboto"
                        fontSize="24"
                        fontWeight="light"
                        fontColor="TitleText"
                        fontLetterSpacing="0"
                    >
                        {step <= 1 && (isWithdraw ? "Withdrawal in progress" : "Deposit in progress")}
                        {step > 1 && (isWithdraw ? "Withdrawal processed" : "Deposit processed")}
                    </Text>
                </ModalTitle>
                <Content>
                    <div>
                        <StyledProcessWrapper>
                            <StyledHorizontalLine />
                            <StyledCircleWrapper index={1}>
                                <StyledCircle completed={step >= 0} />
                                <Text
                                    fontFamily="Roboto"
                                    fontSize="13"
                                    fontWeight="light"
                                    fontColor="TitleText"
                                    fontLetterSpacing="0"
                                >
                                    Initialized
                                </Text>
                            </StyledCircleWrapper>

                            <StyledCircleWrapper index={2}>
                                <StyledCircle completed={step > 0} />
                                <Text
                                    fontFamily="Roboto"
                                    fontSize="13"
                                    fontWeight="light"
                                    fontColor="TitleText"
                                    fontLetterSpacing="0"
                                >
                                    Pending...
                                </Text>
                            </StyledCircleWrapper>

                            <StyledCircleWrapper index={3}>
                                <StyledCircle completed={step > 1} />
                                <Text
                                    fontFamily="Roboto"
                                    fontSize="13"
                                    fontWeight="light"
                                    fontColor="TitleText"
                                    fontLetterSpacing="0"
                                >
                                    Processed!
                                </Text>
                            </StyledCircleWrapper>
                        </StyledProcessWrapper>
                        <TransferAmountWrapper>
                            {(!token || amount === undefined) && !transactionDetailsLoading && (
                                <Text fontSize="16" fontColor="White" fontWeight="light">
                                    Could not get transaction details
                                </Text>
                            )}
                            {(!token || amount === undefined) && transactionDetailsLoading && (
                                <Text fontSize="16" fontColor="White" fontWeight="light">
                                    Fetching transaction details
                                </Text>
                            )}
                            {token && decimalAmount !== undefined && (
                                <>
                                    <Text fontSize="16" fontColor="White" fontWeight="light">
                                        Transfer Amount:&nbsp;
                                    </Text>
                                    <Text fontSize="16" fontColor="White" fontWeight="bold">
                                        <>
                                            {decimalAmount} {isWithdraw ? token?.ccd_name : token?.eth_name}
                                        </>
                                    </Text>
                                </>
                            )}
                        </TransferAmountWrapper>
                    </div>
                    <InfoContainer processed={step > 1}>
                        <Image src={Hourglass.src} width={16.56} height={26.14} alt="Hourglass image" />
                        <Text
                            fontFamily="Roboto"
                            fontSize="13"
                            fontWeight="bold"
                            fontColor="TitleText"
                            fontLetterSpacing="0"
                        >
                            {!props.isWithdraw && (step > 1 ? "Deposit processed!" : "Your deposit is in progress")}
                            {props.isWithdraw && step > 1 && "Withdrawal processed!"}
                            {props.isWithdraw &&
                                step <= 1 &&
                                (props.canWithdraw
                                    ? "Your withdrawal is ready for approval."
                                    : "Your withdrawal is in progress. Please come back later.")}
                        </Text>
                        <Text
                            fontFamily="Roboto"
                            fontSize="11"
                            fontWeight="light"
                            fontColor={status?.isError ? "Red" : "TitleText"}
                            fontLetterSpacing="0"
                            align="center"
                        >
                            <>
                                {status !== undefined && status.message}
                                {status === undefined && (
                                    <>
                                        {step > 1 && "You can now see your finished transaction in History!"}
                                        {step <= 1 &&
                                            !props.isWithdraw &&
                                            "After the transaction is processed you can also check it in your transaction history."}
                                        {props.isWithdraw &&
                                            step <= 1 &&
                                            (props.canWithdraw
                                                ? 'Click "Approve" below to submit your withdrawal approval.'
                                                : "When returning to the bridge, you can return to this view by clicking on the withdrawal in the transaction history.")}
                                    </>
                                )}
                            </>
                        </Text>
                    </InfoContainer>
                    <StyledButtonContainer>
                        {props.isWithdraw && props.canWithdraw ? (
                            <Button variant="primary" onClick={continueHandler}>
                                <div style={{ position: "relative" }}>
                                    <Text fontSize="16" fontColor={"Black"} fontWeight="bold">
                                        Approve
                                    </Text>
                                </div>
                            </Button>
                        ) : (
                            <Button variant="primary" onClick={continueHandler} disabled={disableContinue}>
                                <div style={{ position: "relative" }}>
                                    <Text fontSize="16" fontColor={"Black"} fontWeight="bold">
                                        Continue
                                    </Text>
                                </div>
                            </Button>
                        )}
                    </StyledButtonContainer>
                </Content>
            </StyledContainer>
        </PageWrapper>
    );
};
