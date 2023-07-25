import theme from "src/theme/theme";
import styled from "styled-components";

export const StyledContainer = styled.div`
    width: 500px;
    height: 432px;
    overflow-y: auto;
    background: ${theme.colors.ModalBackground} 0% 0% no-repeat padding-box;
    box-shadow: 0px 6px 10px #00000014;
    border-radius: 6px;

    @media only screen and (max-width: 1050px) {
        width: 100%;
        height: 500px;
    }

    @media only screen and (max-width: 540px) {
        height: 628px;
    }
`;
