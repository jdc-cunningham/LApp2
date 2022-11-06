import React, { useEffect } from 'react';
import './Tabs.scss';

const AppTabs = (props) => {
  const { apps, activeApp, setActiveApp } = props;

  // ehh
  const appTabs = Object.keys(apps).map((appKey, index) => (
    <div
      key={index}
      className={`app__tabs-tab ${apps[appKey] === activeApp ? 'active' : ''}`}
      onClick={() => setActiveApp(apps[appKey])}
    >
      {apps[appKey]}
    </div>
  ));

  return (
    <div className="app__tabs">
      {appTabs}
    </div>
  )
}

export default AppTabs;