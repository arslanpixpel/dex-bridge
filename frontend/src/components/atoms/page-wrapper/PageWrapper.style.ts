import styled from "styled-components";

export const StyledMain = styled.main`
    opacity: 1 !important;
    position: relative;
    margin: 0 auto;
    top: 50%;
    transform: translateY(-50%);
    display: flex;
    flex-direction: column;
    align-items: center;

    @media only screen and (max-width: 1050px) {
        margin: 0 20px;
    }
`;
