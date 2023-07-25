import Container from "@components/atoms/container/Container";
import theme from "src/theme/theme";
import styled from "styled-components";

export const StyledContainer = styled(Container)`
    padding: 27px 40px 40px;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
`;

export const ButtonsContainer = styled.div`
    display: flex;
    & > :first-child {
        margin-right: 8px;
    }
`;

type StyledProcessWrapperProps = {
    strikeThrough: boolean;
};

export const StyledProcessWrapper = styled.div<StyledProcessWrapperProps>`
    position: relative;
    display: inline-flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 4px;

    ${(p) =>
        p.strikeThrough &&
        `
        &::after {
            content: "";
            display: block;
            position: absolute;
            bottom: calc(50% - 1px);
            width: 100%;
            height: 1px;
            background-color: ${theme.colors.Black}
        }
    `}
`;
