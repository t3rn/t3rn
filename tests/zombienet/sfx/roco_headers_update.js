const path = require('path');

async function checkEvent(api) {
  return new Promise(resolve => {

    const eventSection = 'rococoBridge';
    const eventName = 'HeadersAdded';

    api.query.system.events((events) => {
      console.log(events.toHuman())
      const expectedEvents = events
        .toHuman()
        .filter((event) => event.event.section === eventSection && event.event.method === eventName && event.event.data[0] === 'roco');
      
      if (expectedEvents.length > 0) {
        console.log(`✅ Event ${eventSection}.${eventName} emitted for ${expectedEvents[0].event.data}`);
        resolve(1)
      } else {
        console.log(`⏳ Event ${eventSection}.${eventName} for roco not yet emitted.`);
      }
    });
  });
}

async function run(nodeName, networkInfo) {
  const { wsUri, userDefinedTypes } = networkInfo.nodesByName[nodeName];
  const api = await zombie.connect(wsUri, userDefinedTypes);

  const result = await checkEvent(api);
  return result;
}

module.exports = { run };
