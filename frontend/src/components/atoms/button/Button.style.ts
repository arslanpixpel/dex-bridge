import theme from "src/theme/theme";
import styled from "styled-components";
import ButtonShine from "../../../../public/images/button-shine.png";

export const StyledButton = styled.button<{
    variant: "primary" | "secondary" | "connect" | "max";
}>`
    width: ${({ variant }) => (variant === "connect" ? "94px" : variant === "max" ? "38px" : "100%")};
    height: ${({ variant }) => (variant === "connect" ? "26px" : variant === "max" ? "24px" : "40px")};
    min-height: ${({ variant }) => (variant === "connect" ? "26px" : variant === "max" ? "24px" : "40px")};
    border: ${({ variant }) =>
        variant === "primary" || variant === "connect" || variant === "max" ? "none" : "1px solid #000"};
    border-radius: ${({ variant }) => (variant === "max" ? "20px" : "6px")};
    background: ${({ variant }) =>
        variant === "primary"
            ? `${theme.colors.DarkYellow} url(${ButtonShine.src}) right no-repeat padding-box`
            : variant === "connect"
            ? `${theme.colors.Brown}`
            : variant === "max"
            ? "#f1f1f1 0% 0% no-repeat padding-box"
            : "transparent"};
    color: ${({ variant }) => (variant === "max" ? "grey" : "white")};
    cursor: pointer;
    transition: all 0.2s;
    overflow: hidden;

    ${({ variant }) =>
        variant === "connect" &&
        `
    font-size: 10px; color: white;
    font-weight: 300;
    box-shadow: 0px 3px 6px #0000001c;
    font-family: Roboto;
    `}

    ${({ variant }) =>
        variant === "max" &&
        `
      display: flex;
      align-items: center;
      justify-content: center;
      margin-left: auto;
    `}

  ${({ disabled }) =>
        disabled &&
        `
    background-color: gray;
    cursor: not-allowed;
  `}
`;
