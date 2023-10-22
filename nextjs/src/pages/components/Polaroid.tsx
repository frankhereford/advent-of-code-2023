import Image from "next/image";
import Video from './Video'; 


interface PolaroidProps {
  videoIDs: (string | null)[];
  label: string | null;
}


const Polaroid: React.FC<PolaroidProps> = ({ videoIDs, label }) => {
  return (
    <>
      <div className='polaroid'>
        <div className="photograph">
          {videoIDs.map((id, index) => (
            <Video key={index} video_id={id} />
          ))}
          <Image
            alt='televisions in the window of a repair shop'
            src="/televisions_mask.png"
            width={1024}
            height={1024}
          />
        </div>
        <div className='caption'>
          { label }
        </div>
      </div>
    </>
  )
};

export default Polaroid;