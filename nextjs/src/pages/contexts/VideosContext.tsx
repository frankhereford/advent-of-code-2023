// VideosContext.tsx

import React, { createContext, useContext, ReactNode, useState } from 'react';

type VideosContextType = {
  videosLoaded: number;
  incrementVideosLoaded: () => void;
  resetVideosLoaded: () => void;
};

const VideosContext = createContext<VideosContextType | undefined>(undefined);

type VideosProviderProps = {
  children: ReactNode;
};

export const VideosProvider: React.FC<VideosProviderProps> = ({ children }) => {
  const [videosLoaded, setVideosLoaded] = useState<number>(0);

  const incrementVideosLoaded = () => {
    setVideosLoaded(prevCount => prevCount + 1);
  };

  const resetVideosLoaded = () => {
    setVideosLoaded(0);
  };

  return (
    <VideosContext.Provider value={{ videosLoaded, incrementVideosLoaded, resetVideosLoaded }}>
      {children}
    </VideosContext.Provider>
  );
};

export const useVideos = (): VideosContextType => {
  const context = useContext(VideosContext);
  if (!context) {
    throw new Error('useVideos must be used within a VideosProvider');
  }
  return context;
};
