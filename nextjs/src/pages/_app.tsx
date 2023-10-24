import { type AppType } from "next/app";
import { VideosProvider } from './contexts/VideosContext';


import { api } from "~/utils/api";

import "~/styles/globals.css";

const MyApp: AppType = ({ Component, pageProps }) => {
  //return <Component {...pageProps} />;
  return (
    <VideosProvider>
      <Component {...pageProps} />
    </VideosProvider>
  );
};

export default api.withTRPC(MyApp);
