import { useRouter } from "next/router";
import { forwardRef, useCallback, useImperativeHandle, useState } from "react";
import styled from "styled-components";
import { BridgeDirection, routes } from "src/constants/routes";
import theme from "src/theme/theme";

const Button = styled.button`
    position: absolute;
    right: 30px;
    top: 30px;
    font-size: 12px;
    border-radius: 15px;
    overflow: hidden;
    white-space: nowrap;
    color: ${theme.colors.White};
    background-color: ${theme.colors.Green};
    border: none;
    cursor: pointer;
    width: 30px;
    height: 30px;
    transition: all ease-out 0.2s;

    &:hover {
        width: 200px;
        padding-right: 40px;
        padding-left: 15px;
    }
`;

const ButtonIcon = styled.span`
    border-radius: inherit;
    height: inherit;
    width: 30px;
    text-align: center;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    background-color: inherit;
    position: absolute;
    top: 0;
    right: 0;
    font-size: 14px;
`;

export type NotifyPendingWithdrawalRef = {
    /**
     * Shows a `window.confirm` to the user, notifying that a withdrawal is ready for approval. Will block subsequent calls if one is already open.
     */
    open(): void;
};

type Props = {
    hasPendingWithdrawal: boolean;
};

// eslint-disable-next-line react/display-name
const NotifyPendingWithdrawal = forwardRef<NotifyPendingWithdrawalRef, Props>(({ hasPendingWithdrawal }, ref) => {
    const { push } = useRouter();
    const [isOpen, setIsOpen] = useState(false);

    const requestNavigateToHistory = useCallback(() => {
        if (isOpen) {
            return;
        }

        setIsOpen(true);
        const goToHistory = window.confirm(
            'You have a pending withdrawal, which can now be approved.\n\nPress "OK" to view your transaction history.'
        );
        setIsOpen(false);

        if (goToHistory) {
            push(routes.history(BridgeDirection.Withdraw));
        }
    }, [push, isOpen]);

    useImperativeHandle(
        ref,
        () => ({
            open: requestNavigateToHistory,
        }),
        [requestNavigateToHistory]
    );

    if (!hasPendingWithdrawal) {
        return null;
    }

    return (
        <Button onClick={requestNavigateToHistory}>
            Withdraw ready for approval<ButtonIcon>!</ButtonIcon>
        </Button>
    );
});

export default NotifyPendingWithdrawal;
