import React, { useEffect, useState } from 'react';
import './index.scss';

const App = () => {
  const [time, setTime] = useState(new Date());

  useEffect(() => {
    const intervalId = setInterval(() => {
      setTime(new Date());
    }, 1000);
    
    return () => clearInterval(intervalId);
  }, []);

  const formatTime = (time) => {
    return time.toLocaleTimeString();
  };

  return (
    <div className="clock-container">
      <div className="clock">
        {formatTime(time)}
      </div>
    </div>
  );
};

export default App;