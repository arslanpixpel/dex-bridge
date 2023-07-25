import React from "react";
import { StyledButton } from "./Button.style";

interface Props extends React.ButtonHTMLAttributes<HTMLButtonElement> {
    variant: "primary" | "secondary" | "connect" | "max";
    onClick?: () => void;
}

const Button = ({ children, variant, disabled, onClick, ...rest }: Props): JSX.Element => {
    return (
        <StyledButton variant={variant} disabled={disabled} onClick={onClick} {...rest}>
            {children}
        </StyledButton>
    );
};

export default Button;
