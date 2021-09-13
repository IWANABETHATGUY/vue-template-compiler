//! This module defines a collection of flags used for Vue's runtime.
//! Currently it includes preamble helper and vnode patch flags.
//! Ideally we can make flags extensible by extracting them to trait.
//! But currently it works well enough and adding traits makes compiler
//! bloated with too many generic parameters.

use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    /// Patch flags are optimization hints generated by the compiler.
    /// when a block with dynamicChildren is encountered during diff, the algorithm
    /// enters "optimized mode". In this mode, we know that the vdom is produced by
    /// a render function generated by the compiler, so the algorithm only needs to
    /// handle updates explicitly marked by these patch flags.
    ///
    /// Check the `patchElement` function in '../../runtime-core/src/renderer.ts' to see how the
    /// flags are handled during diff.
    pub struct PatchFlag: i32 {
        /// Indicates an element with dynamic textContent (children fast path)
        const TEXT = 1;
        /// Indicates an element with dynamic class binding.
        const CLASS = 1 << 1;

        /// Indicates an element with dynamic style
        /// The compiler pre-compiles static string styles into static objects
        /// and detects and hoists inline static objects
        /// e.g. style="color: red" and :style="{ color: 'red' }" both get hoisted as
        ///   const style = { color: 'red' }
        ///   render() { return e('div', { style }) }
        const STYLE = 1 << 2;

        /// Indicates an element that has non-class/style dynamic props.
        /// Can also be on a component that has any dynamic props (includes
        /// class/style). when this flag is present, the vnode also has a dynamicProps
        /// array that contains the keys of the props that may change so the runtime
        /// can diff them faster (without having to worry about removed props)
        const PROPS = 1 << 3;

        /// Indicates an element with props with dynamic keys. When keys change, a full
        /// diff is always needed to remove the old key. This flag is mutually
        /// exclusive with CLASS, STYLE and PROPS.
        const FULL_PROPS = 1 << 4;

        /// Indicates an element with event listeners (which need to be attached during hydration)
        const HYDRATE_EVENTS = 1 << 5;

        /// Indicates a fragment whose children order doesn't change.
        const STABLE_FRAGMENT = 1 << 6;

        /// Indicates a fragment with keyed or partially keyed children
        const KEYED_FRAGMENT = 1 << 7;

        /// Indicates a fragment with unkeyed children.
        const UNKEYED_FRAGMENT = 1 << 8;

        /// Indicates an element that only needs non-props patching, e.g. ref or
        /// directives (onVnodeXXX hooks). since every patched vnode checks for refs
        /// and onVnodeXXX hooks, it simply marks the vnode so that a parent block
        /// will track it.
        const NEED_PATCH = 1 << 9;

        /// Indicates a component with dynamic slots (e.g. slot that references a v-for
        /// iterated value, or dynamic slot names).
        /// Components with this flag are always force updated.
        const DYNAMIC_SLOTS = 1 << 10;

        /// Indicates a fragment that was created only because the user has placed
        /// comments at the root level of a template. This is a dev-only flag since
        /// comments are stripped in production.
        const DEV_ROOT_FRAGMENT = 1 << 11;

        /// SPECIAL FLAGS -------------------------------------------------------------
        /// Special flags are negative integers. They are never matched against using
        /// bitwise operators (bitwise matching should only happen in branches where
        /// patchFlag > 0), and are mutually exclusive. When checking for a special
        /// flag, simply check patchFlag === FLAG.

        /// Indicates a hoisted static vnode. This is a hint for hydration to skip
        /// the entire sub tree since static content never needs to be updated.
        const HOISTED = -1;
        /// A special flag that indicates that the diffing algorithm should bail out
        /// of optimized mode. For example, on block fragments created by renderSlot()
        /// when encountering non-compiler generated slots (i.e. manually written
        /// render functions, which should always be fully diffed)
        /// OR manually cloneVNodes
        const BAIL = -2;
    }
}

/// Static level describes how much an IR node can be statically generated.
/// Higher levels implies lower levels. e.g. a node that can be stringified
/// can always be hoisted and skipped for patch.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum StaticLevel {
    NotStatic,
    CanSkipPatch,
    CanHoist,
    CanStringify,
}

#[derive(Clone, Copy)]
pub enum RuntimeHelper {
    Fragment,
    Teleport,
    Suspense,
    KeepAlive,
    BaseTransition,
    OpenBlock,
    CreateBlock,
    CreateElementBlock,
    CreateVnode,
    CreateElementVnode,
    CreateComment,
    CreateText,
    CreateStatic,
    ResolveComponent,
    ResolveDynamicComponent,
    ResolveDirective,
    ResolveFilter,
    WithDirectives,
    RenderList,
    RenderSlot,
    CreateSlots,
    ToDisplayString,
    MergeProps,
    NormalizeClass,
    NormalizeStyle,
    NormalizeProps,
    GuardReactiveProps,
    ToHandlers,
    Camelize,
    Capitalize,
    ToHandlerKey,
    SetBlockTracking,
    PushScopeId,
    PopScopeId,
    WithScopeId,
    WithCtx,
    Unref,
    IsRef,
    WithMemo,
    IsMemoSame,
}

/// PreambleHelper is a collection of JavaScript imports at the head of output
/// e.g. v-for needs a list looping helper to make vdom
/// preamble helper needs collect helper when traversing template ast
/// and generates corresponding JavaScript imports in compilation output
impl RuntimeHelper {
    pub fn generate_imports(&self) -> String {
        todo!()
    }
    pub fn helper_str(&self) -> &'static str {
        use RuntimeHelper::*;
        match *self {
            Fragment => "Fragment",
            Teleport => "Teleport",
            Suspense => "Suspense",
            KeepAlive => "KeepAlive",
            BaseTransition => "BaseTransition",
            OpenBlock => "openBlock",
            CreateBlock => "createBlock",
            CreateElementBlock => "createElementBlock",
            CreateVnode => "createVnode",
            CreateElementVnode => "createElementVnode",
            CreateComment => "createComment",
            CreateText => "createText",
            CreateStatic => "createStatic",
            ResolveComponent => "resolveComponent",
            ResolveDynamicComponent => "resolveDynamicComponent",
            ResolveDirective => "resolveDirective",
            ResolveFilter => "resolveFilter",
            WithDirectives => "withDirectives",
            RenderList => "renderList",
            RenderSlot => "renderSlot",
            CreateSlots => "createSlots",
            ToDisplayString => "toDisplayString",
            MergeProps => "mergeProps",
            NormalizeClass => "normalizeClass",
            NormalizeStyle => "normalizeStyle",
            NormalizeProps => "normalizeProps",
            GuardReactiveProps => "guardReactiveProps",
            ToHandlers => "toHandlers",
            Camelize => "camelize",
            Capitalize => "capitalize",
            ToHandlerKey => "toHandlerKey",
            SetBlockTracking => "setBlockTracking",
            PushScopeId => "pushScopeId",
            PopScopeId => "popScopeId",
            WithScopeId => "withScopeId",
            WithCtx => "withCtx",
            Unref => "unref",
            IsRef => "isRef",
            WithMemo => "withMemo",
            IsMemoSame => "isMemoSame",
        }
    }
}

/*
// we can extend helper by extracting trait like below.
// but it does not pay off now.
pub trait PreambleHelper {
    fn generate_imports(&self) -> String;
    fn helper_str(&self) -> &'static str;
}
*/
