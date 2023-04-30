const { exec, spawn } = require('child_process');

console.log("Installing client/packages with make all ⏳")
exec('make all', { cwd: '../../../client/packages' }, (err, stdout, stderr) => {
    if (err) {
        console.error(err);
        return;
    }
    console.log("client/packages:: make all done - client packages assumed to be installed ✅")
    console.log("Starting executor ⏳")
    let executor_process = spawn('make', ['start_executor'], { cwd: '../../../client/packages' });

    executor_process.stdout.on('data', (data) => {
        console.log(`executor output: ${data}`);
    });

    executor_process.stderr.on('data', (err) => {
        // console.log("Executor crashed ❌")
        console.error(`executor stderr: ${err}`);
        // throw err
    });

    executor_process.on('close', (code) => {
        console.log(`executor exited with code ${code}`);
        process.exit(code)
    });

    console.log("Running make::test_confirm_xtx sending single SFX on top of Rococo header ranges ⏳")
    exec('make test_confirm_xtx', { cwd: '../../../client/packages' }, (err, stdout, stderr) => {
        if (err) {
            // Some error occurred
            console.log("Test confirm SFX failed ❌")
            console.error(err);
            executor_process.kill('SIGINT')
            process.exit(1)
        } else {
            console.log("Test confirm SFX passed ✅")
            // Sleep to wait for executor's confirmation output to be printed
            setTimeout(() => {
                // Kill executor with success code
                console.log("Killing executor with success code ✅")
                executor_process.kill(0)
                console.log("Exiting process with success code ✅")
                process.exit(0)
            }, 5000)
        }
    });
});
