import React, { InputHTMLAttributes } from "react";
import { StyledInput } from "./input.style";

interface Props extends InputHTMLAttributes<HTMLInputElement> {
    valid: boolean;
}

const Input = (props: Props) => {
    return <StyledInput {...props} />;
};

export default Input;
