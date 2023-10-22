import Image from "next/image";
import Video from '~/pages/components/Video'; 
import React, { useEffect, useRef } from "react";


interface PolaroidProps {
  videoIDs: (string | null)[];
  label: string | null;
}


const Polaroid: React.FC<PolaroidProps> = ({ videoIDs, label }) => {
  
  const photographRef = useRef<HTMLDivElement | null>(null);
  const captionRef = useRef<HTMLDivElement | null>(null);

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
            className='televisions'
            alt='televisions in the window of a repair shop'
            src="/televisions_mask.png"
            width={1024}
            height={1024}
          />
          <Image
            className='polaroid_texture'
            alt='texture'
            src="/polaroid_texture.png"
            width={1024}
            height={1024}
          />
        </div>
        <div ref={captionRef} className='caption'>
          { label }
        </div>
      </div>
    </>
  )
};

export default Polaroid;