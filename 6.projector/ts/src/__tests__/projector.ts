import { Operation } from "../config";
import Projector from "../projector"

function getData() {
    return {
        projector: {
            "/": {
                "foo": "bar1",
                "fem": "is great",
            },
            "/foo": { "foo": "bar2" },
            "/foo/bar": { "foo": "bar3" },
        },
    }
}

function getProjector(pwd: string, data = getData()): Projector {
    return new Projector({
        args: [],
        operation: Operation.Print,
        config: "hello world!", // actual path
        pwd: pwd,
    }, data);
}

test("getValueAll", function() {
    const proj = getProjector("/foo/bar")
    expect(proj.getValueAll()).toEqual({
        "fem": "is great",
        "foo": "bar3",
    })
})

test("getValue", function() {
    let proj = getProjector("/foo/bar")
    expect(proj.getValue("foo")).toEqual("bar3")
    proj = getProjector("/foo")
    expect(proj.getValue("foo")).toEqual("bar2")
    expect(proj.getValue("fem")).toEqual("is great")
})

test("setValue", function() {
    const data = getData()

    getProjector("/foo", data).setValue("baz", "abc")
    getProjector("/foo/bar", data).setValue("fem", "is better than great")

    let proj = getProjector("/foo", data)
    expect(proj.getValue("foo")).toEqual("bar2")
    expect(proj.getValue("fem")).toEqual("is great")
    expect(proj.getValue("baz")).toEqual("abc")
    proj = getProjector("/foo/bar", data)
    expect(proj.getValue("foo")).toEqual("bar3")
    expect(proj.getValue("baz")).toEqual("abc")
    expect(proj.getValue("fem")).toEqual("is better than great")
    proj = getProjector("/", data)
    expect(proj.getValue("baz")).toEqual(undefined)
    expect(proj.getValue("foo")).toEqual("bar1")
    expect(proj.getValue("fem")).toEqual("is great")
})

test("removeValue", function() {
    const data = getData()

    getProjector("/foo", data).setValue("baz", "abc")
    getProjector("/foo/bar", data).setValue("fem", "is better than great")

    let proj = getProjector("/foo/bar", data)
    proj.removeValue("fem")
    proj.removeValue("foo")

    expect(proj.getValue("fem")).toEqual("is great")
    expect(proj.getValue("foo")).toEqual("bar2")

    proj = getProjector("/foo", data)
    expect(proj.getValue("fem")).toEqual("is great")
    expect(proj.getValue("foo")).toEqual("bar2")
})
