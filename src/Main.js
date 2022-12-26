import React, { useState, useEffect } from 'react';
import init, { monkey_talk } from 'monkey-wasm';
import './Main.css';

const prompt  = ">";
const width   = 30;
const hello   = "Hi! I'm Charlie!"

const Main = () => {
  const [quote, setQuote] = useState(null);
  const [monkey, setMonkey] = useState("not ready..");

  useEffect(() => {
    init().then(() => {
      setMonkey(
        monkey_talk(quote ? quote : hello, width)
      );
    })
  }, [quote]);

  const handleQuote = (e) => {
    const q = e.target.value;
    setQuote(q);
  }

  return (
    <div className="Main">
      <>
        <div className="textBox">
          <code><pre>
          {monkey}
          </pre></code>
        </div>
        <div className="promptBox">
          <div className="prompt">{prompt}</div>
          <input
            value={quote}
            className="promptInput"
            onChange={handleQuote}
            autoFocus
          />
        </div>
      </>
    </div>
  );
}

export default Main;
