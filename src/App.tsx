import { useState } from 'react';

import { Serializer } from './logic-pad/src/data/serializer/allSerializers';
import { Compressor } from './logic-pad/src/data/serializer/compressor/allCompressors';

function App() {
  const [url, setUrl] = useState<string>("");

  const deserialize = async () => {
    const value = url.split("?d=")[1];
    const decompressed = await Compressor.decompress(value);
    const puzzle = Serializer.parsePuzzle(decompressed);
    console.log(puzzle);
  };

  return (
    <>
      <div>
        <input type="text" value={url} onChange={e => setUrl(e.target.value)} />
        <input type="button" value="deserialize" onClick={deserialize} />
      </div>
    </>
  )
}

export default App
