import type { Metadata } from "next";
import "./globals.css";
import Header from "@/components/layout/Header";
import CosmosChainProvider from "@/components/provider/CosmosChainProvider";
import "@interchain-ui/react/styles";
import { ConfigProvider } from "antd";
import { THEME } from "@/styles/theme";
import { FONT } from "@/styles/font";

export const metadata: Metadata = {
  title: "Create Next App",
  description: "Generated by create next app",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={`${FONT.className} bg-secondary text-text px-60`}>
        <CosmosChainProvider>
          <ConfigProvider
            theme={{
              token: {
                colorText: THEME.TEXT_COLOR,
                colorPrimary: THEME.PRIMARY_COLOR,
                fontFamily: FONT.style.fontFamily
              },
              components: {
                Menu: {
                  itemBg: 'transparent',
                },
                Button: {
                  defaultBg: THEME.BUTTON_COLOR,
                  defaultHoverBg: THEME.BUTTON_HOVER_COLOR,
                  defaultActiveBg: THEME.BUTTON_HOVER_COLOR,
                  lineWidth: 0,
                  fontWeight: 'inherit',
                  fontSize: 'inherit',
                  colorPrimaryText: THEME.SECONDARY_COLOR,
                },
                Modal: {
                  contentBg: 'transparent',
                },
                Typography: {
                  fontSize: 'inherit',
                  colorText: 'currentColor',
                },
                Input: {
                  colorBgContainer: THEME.BUTTON_COLOR
                }
              }
            }}
          >
            <Header />
            {children}
          </ConfigProvider>
        </CosmosChainProvider>
      </body>
    </html>
  );
}
