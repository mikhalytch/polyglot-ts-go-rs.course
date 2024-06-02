import getConfig, { Operation } from "../config"


test("simple print all", function() {
    const conf = getConfig({});
    expect(conf.operation).toEqual(Operation.Print);
    expect(conf.args).toEqual([]);
});

test("print key", function() {
    const conf = getConfig({
        args: ["foo"],
    });
    expect(conf.operation).toEqual(Operation.Print);
    expect(conf.args).toEqual(["foo"]);
});

test("add key-value", function() {
    const conf = getConfig({
        args: ["add", "foo", "bar"],
    });
    expect(conf.operation).toEqual(Operation.Add);
    expect(conf.args).toEqual(["foo", "bar"]);
});

test("remove key", function() {
    const conf = getConfig({
        args: ["remove", "foo"],
    });
    expect(conf.operation).toEqual(Operation.Remove);
    expect(conf.args).toEqual(["foo"]);
});
