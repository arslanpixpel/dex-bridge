import React, { FC, useEffect } from "react";
import { LogoWrapper } from "./Logo.style";
import ConcordiumLogoImage from "../../../../public/icons/Concordium_Logo.svg";
import CornucopiaLogoImage from "../../../../public/images/Cornucopia-black@3x.png";
import Image from "next/image";
import Text from "@components/atoms/text/text";
import packageJson from "package.json";
import { useRouter } from "next/router";
import { routes } from "src/constants/routes";

interface Props {
    isTablet: boolean;
}

export const ConcordiumLogo: FC<Props> = ({ isTablet }) => (
    <LogoWrapper logo="ccd">
        <Text
            fontFamily="Roboto"
            fontSize={isTablet ? "9" : "11"}
            fontWeight="regular"
            fontColor="Black"
            fontLetterSpacing="0"
        >
            v{packageJson.version} | Powered by
        </Text>
        <Image src={ConcordiumLogoImage.src} width={isTablet ? 70 : 104} height={isTablet ? 12 : 18} alt="Concordium" />
    </LogoWrapper>
);

export const CornucopiaLogo: FC<Props> = ({ isTablet }) => {
    const { push, prefetch } = useRouter();

    useEffect(() => {
        prefetch(routes.deposit.path);
    }, [prefetch]);

    return (
        <LogoWrapper logo="ccp" onClick={() => push(routes.deposit.path)} role="button">
            <Image
                src={CornucopiaLogoImage.src}
                width={isTablet ? 130 : 241}
                height={isTablet ? 22 : 41}
                alt="Cornucopia Logo"
            />
        </LogoWrapper>
    );
};
