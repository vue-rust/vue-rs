import Vue from 'vue'

let id = 0;

export default function (type, options) {
    id += 1;

    const key = '!rs' + id;

    const watch = {}
    const computed = {}

    options
        .props
        .forEach(p => {
            computed['c' + p] = {
                get () { return this['d' + p] },
                set (v) { this[key][p] = v }
            }

            watch[p] = {
                deep: true,
                handler (n) { this[key][p] = n },
            }
        })

    return {
        activated () {
            this[key].componentActive = true
        },
        beforeDestroy () {
            this[key].componentActive = false
            this[key].free()
        },
        computed,
        data () {
            return { [key]: type }
        },
        deactivated () {
            this[key].componentActive = false
        },
        mounted () {
            this[key].data = this
            this[key].componentActive = true
        },
        props: options.props,
        watch
    }
};