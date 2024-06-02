import getOpts from "./opts";
import getConfig, { Operation, } from "./config";
import Projector from "./projector";

const opts = getOpts();
const config = getConfig(opts);
const proj = Projector.fromConfig(config);

switch (config.operation) {
    case Operation.Print:
        if (config.args.length == 0) {
            console.log(JSON.stringify(proj.getValueAll()));
        } else {
            const key = config.args[0];
            const value = proj.getValue(key);
            if (value) {
                console.log(value);
            }
        }
        break;
    case Operation.Add:
        proj.setValue(config.args[0], config.args[1]);
        proj.save();
        break;
    case Operation.Remove:
        proj.removeValue(config.args[0]);
        proj.save();
        break;
    default:
        console.log("unknown operation", config.operation);
}

