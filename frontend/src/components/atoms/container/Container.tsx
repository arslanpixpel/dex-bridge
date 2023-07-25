import React from "react";
import { StyledContainer } from "./Container.style";

interface Props {
    children?: React.ReactNode;
    className?: string;
}

const Container = ({ children, className }: Props): JSX.Element => {
    return <StyledContainer className={className}>{children}</StyledContainer>;
};

export default Container;
