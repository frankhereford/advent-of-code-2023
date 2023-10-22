import { Html, Head, Main, NextScript } from 'next/document'

export default function Document() {
  return (
    <Html data-theme="cmyk">
      <Head>
        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
        <link href="https://fonts.googleapis.com/css2?family=Nothing+You+Could+Do&family=Patrick+Hand&display=swap" rel="stylesheet" />
      </Head>
      <body>
        <Main />
        <NextScript />
      </body>
    </Html>
  )
}
