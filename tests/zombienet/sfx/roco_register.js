const { exec } = require('child_process');
const path = require('path');

async function checkEvent(api) {
  return new Promise(resolve => {
    const eventSection = 'xdns';
    const eventName = 'GatewayRecordStored';

    api.query.system.events((events) => {
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

  // Execute cli to submit roco registration
  const command = 'pnpm cli register -g roco-local';
  const cliPath = path.join(__dirname, '../../../client/packages/cli');

  try {
    exec(command, { cwd: cliPath }, (error) => {
      if (error) {
        console.error(`Error executing command "${command}": ${error}`);
        process.exit(1)
      }
    });
  } catch (err) {
    console.error(err);
    process.exit(1)
  }

  const result = await checkEvent(api);
  return result;
}

module.exports = { run };
