import Button from "@components/atoms/button/Button";
import PageWrapper from "@components/atoms/page-wrapper/PageWrapper";
import Image from "next/image";
import { useRouter } from "next/router";
import { FC, PropsWithChildren, ReactNode, useState } from "react";
import { routes } from "src/constants/routes";
import { useTransactionFlowStore } from "src/store/transaction-flow";
import ConcordiumIcon from "../../../../public/icons/concordium-icon.svg";
import EthereumIcon from "../../../../public/icons/ethereum-icon.svg";
import Text from "../../atoms/text/text";
import { ButtonsContainer, StyledContainer, StyledProcessWrapper } from "./TransferOverview.style";

type TransferOverviewLineProps = {
    isEth?: boolean;
    title: ReactNode;
    details: ReactNode;
    completed?: boolean;
};

export const TransferOverviewLine: FC<TransferOverviewLineProps> = ({
    isEth = false,
    title,
    details,
    completed = false,
}) => (
    <StyledProcessWrapper strikeThrough={completed}>
        <Image
            src={isEth ? EthereumIcon.src : ConcordiumIcon.src}
            alt={isEth ? "Ethereum Icon" : "Concordium Icon"}
            height="20"
            width="20"
        />
        <Text fontFamily="Roboto" fontSize="11" fontWeight="light" fontColor="TitleText" fontLetterSpacing="0">
            {title}
        </Text>
        <Text fontFamily="Roboto" fontSize="11" fontWeight="bold" fontColor="TitleText" fontLetterSpacing="0">
            {details}
        </Text>
    </StyledProcessWrapper>
);

type TransferOverviewStatus = {
    message: string;
    isError: boolean;
};

export const useTransferOverviewStatusState = () => {
    const [status, setStatus] = useState<TransferOverviewStatus>();
    const setError = (message: string) => setStatus({ isError: true, message });
    const setInfo = (message: string) => setStatus({ isError: false, message });

    return {
        status,
        setError,
        setInfo,
    };
};

type Props = {
    /**
     * Callback function for handling submission for specific flow.
     * Expects hash of the transaction to be returned, or undefined if an error happened.
     */
    handleSubmit(): Promise<string | undefined>;
    /**
     * Status message to be shown on page. Can either be shown as an error, or plain information.
     */
    status?: TransferOverviewStatus;
    /**
     * Title of the overview page
     */
    title: string;
    /**
     * Estimated time required to complete flow in text.
     */
    timeToComplete: string;
    /**
     * When true, will show a prompt when pressing cancel,
     * warning the user that there is a pending signature request in the corresponding wallet.
     */
    pendingWalletSignature: boolean;
    isDeposit?: boolean;
};

export const TransferOverview: FC<PropsWithChildren<Props>> = ({
    handleSubmit,
    status,
    title,
    timeToComplete,
    pendingWalletSignature,
    isDeposit = false,
    children,
}) => {
    const [pendingSubmission, setPendingSubmission] = useState(false);
    const { setTransactionHash } = useTransactionFlowStore();
    const { back, push } = useRouter();

    const submit = async () => {
        setPendingSubmission(true);
        const hash = await handleSubmit();
        setPendingSubmission(false);

        if (hash) {
            const nextRoute = isDeposit ? routes.deposit.tx(hash) : routes.withdraw.tx(hash);
            setTransactionHash(hash);
            push(nextRoute);
        }
    };

    const cancel = () => {
        if (pendingWalletSignature) {
            const confirmed = window.confirm(
                `There is a pending wallet signature in your ${
                    isDeposit ? "Ethereum" : "Concordium"
                } wallet, which can be safely rejected when cancelling this flow.`
            );

            if (!confirmed) {
                return;
            }
        }

        back();
    };

    return (
        <PageWrapper>
            <StyledContainer>
                <Text fontFamily="Roboto" fontSize="24" fontWeight="light" fontColor="TitleText" fontLetterSpacing="0">
                    {title}
                </Text>
                <div>
                    <Text
                        fontFamily="Roboto"
                        fontSize="13"
                        fontWeight="bold"
                        fontColor="TitleText"
                        fontLetterSpacing="0"
                    >
                        {timeToComplete}
                    </Text>
                    <div style={{ marginTop: 12 }} />
                    <Text
                        fontFamily="Roboto"
                        fontSize="13"
                        fontWeight="light"
                        fontColor="TitleText"
                        fontLetterSpacing="0"
                    >
                        Transactions required:
                    </Text>

                    <div style={{ marginTop: 16 }} />
                    {children}
                </div>
                <Text fontSize="12" fontWeight="light" fontColor={status?.isError ? "Red" : "Black"} align="center">
                    {status ? status.message : <>&nbsp;</>}
                </Text>
                <ButtonsContainer>
                    <Button variant="secondary" onClick={cancel}>
                        <div style={{ position: "relative" }}>
                            <Text fontSize="16" fontColor="Black" fontWeight="bold">
                                Cancel
                            </Text>
                        </div>
                    </Button>
                    <Button variant="primary" disabled={pendingSubmission} onClick={submit}>
                        <div style={{ position: "relative" }}>
                            <Text fontSize="16" fontColor="Black" fontWeight="bold">
                                Continue
                            </Text>
                        </div>
                    </Button>
                </ButtonsContainer>
            </StyledContainer>
        </PageWrapper>
    );
};
