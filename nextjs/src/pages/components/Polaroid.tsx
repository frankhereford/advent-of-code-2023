import Image from "next/image";
import Video from '~/pages/components/Video'; 
import React, { useEffect, useRef } from "react";
import { useVideos } from '~/pages/contexts/VideosContext';

interface PolaroidProps {
  videoIDs: (string | undefined)[];
  label: string | undefined;
}

const Polaroid: React.FC<PolaroidProps> = ({ videoIDs, label }) => {
  
  const photographRef = useRef<HTMLDivElement | null>(null);
  const captionRef = useRef<HTMLDivElement | null>(null);
  const { videosLoaded, incrementVideosLoaded, resetVideosLoaded } = useVideos();

  const adjustFontSize = () => {
    if (photographRef.current && captionRef.current) {
      // Get the height of the parent div
      const photographHeight = photographRef.current.offsetHeight;

      const fontSize = photographHeight * 0.06;
      const margin = photographHeight * 0.04;
      captionRef.current.style.fontSize = `${fontSize}px`;
      captionRef.current.style.marginTop = `${margin}px`;
      captionRef.current.style.marginBottom = `${margin}px`;
    }
  };

  useEffect(() => {
    // Adjust font size initially
    adjustFontSize();

    // Set up the event listener
    window.addEventListener("resize", adjustFontSize);

    // Clean up the event listener when the component unmounts
    return () => {
      window.removeEventListener("resize", adjustFontSize);
    };
  }, []);

  return (
    <>
      <div className='polaroid'>
        <div ref={photographRef} className="photograph">
          {videoIDs.map((id, index) => (
            <Video key={index} videoId={id} />
          ))}
          <Image
            alt='televisions in the window of a repair shop'
            src="/televisions_mask.png"
            width={1024}
            height={1024}
          />
        </div>
        <div ref={captionRef} className='caption'>
          <div className={`fade-in ${videosLoaded >= 2 ? 'visible' : ''}`}>
            { label }
          </div>
        </div>
        <div className='image-credit'>Photograph by <a className='thin-underline' href='https://en.wikipedia.org/wiki/Zoe_Leonard'>Zoe Leonard</a></div>
      </div>
    </>
  )
};

export default Polaroid;