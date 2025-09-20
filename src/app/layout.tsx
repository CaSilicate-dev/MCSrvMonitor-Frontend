'use client';
import '@ant-design/v5-patch-for-react-19';

import { ConfigProvider, Button, theme, App } from 'antd';
import React, { useState } from 'react';

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {

  return (
    <App>
      <ConfigProvider>
        <html lang="en">
          <body style={{
          }}>
            
              {children}
            
          </body>
        </html>
      </ConfigProvider>
    </App>

  );
}
