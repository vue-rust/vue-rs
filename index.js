import Vue from 'vue'

let id = 0;

export default function (type, getProps, getMethods) {
    id += 1;

    const rust = '!rs' + id;

    const watch = {}
    const computed = {}
    const methods = {};
    const props = getProps()

    props
        .forEach(p => {
            computed['c' + p] = {
                get () { return this['d' + p] },
                set (v) { this[rust][p] = v }
            }

            watch[p] = {
                deep: true,
                handler (n) { this[rust][p] = n },
            }
        })

    if (getMethods) {
        getMethods.forEach(a => {
            methods[a[0]] = function () {
                this[rust][a].apply(this[rust], arguments)
            }
        })
    }

    return {
        activated () {
            this[rust].componentActive = true
        },
        beforeDestroy () {
            this[rust].componentActive = false
            this[rust].free()
        },
        computed,
        data () {
            return { [rust]: type() }
        },
        deactivated () {
            this[rust].componentActive = false
        },
        methods,
        mounted () {
            this[rust].data = this
            this[rust].componentActive = true
        },
        props,
        watch
    }
};