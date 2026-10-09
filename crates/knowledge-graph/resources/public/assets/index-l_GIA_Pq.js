import{r as e,t}from"./rolldown-runtime-QTnfLwEv.js";import{a as n,i as r,n as i,r as a,t as o}from"./vendor-json-view-XsbV5yWD.js";import{a as s,c,d as l,f as u,h as d,i as f,l as p,m,n as h,o as g,p as _,r as v,s as y,t as b,u as x}from"./vendor-xyflow-Dg2A4cNM.js";import{a as S,i as C,n as w,o as ee,r as T,t as E}from"./vendor-router-DrFk6dZO.js";import{n as D,r as O,t as k}from"./vendor-markdown-Di8s8vbi.js";import{n as A,r as te,t as ne}from"./vendor-panels-Cn6VYsgu.js";(function(){let e=document.createElement(`link`).relList;if(e&&e.supports&&e.supports(`modulepreload`))return;for(let e of document.querySelectorAll(`link[rel="modulepreload"]`))n(e);new MutationObserver(e=>{for(let t of e)if(t.type===`childList`)for(let e of t.addedNodes)e.tagName===`LINK`&&e.rel===`modulepreload`&&n(e)}).observe(document,{childList:!0,subtree:!0});function t(e){let t={};return e.integrity&&(t.integrity=e.integrity),e.referrerPolicy&&(t.referrerPolicy=e.referrerPolicy),e.crossOrigin===`use-credentials`?t.credentials=`include`:e.crossOrigin===`anonymous`?t.credentials=`omit`:t.credentials=`same-origin`,t}function n(e){if(e.ep)return;e.ep=!0;let n=t(e);fetch(e.href,n)}})();var re=t((e=>{function t(e,t){var n=e.length;e.push(t);a:for(;0<n;){var r=n-1>>>1,a=e[r];if(0<i(a,t))e[r]=t,e[n]=a,n=r;else break a}}function n(e){return e.length===0?null:e[0]}function r(e){if(e.length===0)return null;var t=e[0],n=e.pop();if(n!==t){e[0]=n;a:for(var r=0,a=e.length,o=a>>>1;r<o;){var s=2*(r+1)-1,c=e[s],l=s+1,u=e[l];if(0>i(c,n))l<a&&0>i(u,c)?(e[r]=u,e[l]=n,r=l):(e[r]=c,e[s]=n,r=s);else if(l<a&&0>i(u,n))e[r]=u,e[l]=n,r=l;else break a}}return t}function i(e,t){var n=e.sortIndex-t.sortIndex;return n===0?e.id-t.id:n}if(e.unstable_now=void 0,typeof performance==`object`&&typeof performance.now==`function`){var a=performance;e.unstable_now=function(){return a.now()}}else{var o=Date,s=o.now();e.unstable_now=function(){return o.now()-s}}var c=[],l=[],u=1,d=null,f=3,p=!1,m=!1,h=!1,g=!1,_=typeof setTimeout==`function`?setTimeout:null,v=typeof clearTimeout==`function`?clearTimeout:null,y=typeof setImmediate<`u`?setImmediate:null;function b(e){for(var i=n(l);i!==null;){if(i.callback===null)r(l);else if(i.startTime<=e)r(l),i.sortIndex=i.expirationTime,t(c,i);else break;i=n(l)}}function x(e){if(h=!1,b(e),!m)if(n(c)!==null)m=!0,S||(S=!0,D());else{var t=n(l);t!==null&&A(x,t.startTime-e)}}var S=!1,C=-1,w=5,ee=-1;function T(){return g?!0:!(e.unstable_now()-ee<w)}function E(){if(g=!1,S){var t=e.unstable_now();ee=t;var i=!0;try{a:{m=!1,h&&(h=!1,v(C),C=-1),p=!0;var a=f;try{b:{for(b(t),d=n(c);d!==null&&!(d.expirationTime>t&&T());){var o=d.callback;if(typeof o==`function`){d.callback=null,f=d.priorityLevel;var s=o(d.expirationTime<=t);if(t=e.unstable_now(),typeof s==`function`){d.callback=s,b(t),i=!0;break b}d===n(c)&&r(c),b(t)}else r(c);d=n(c)}if(d!==null)i=!0;else{var u=n(l);u!==null&&A(x,u.startTime-t),i=!1}}break a}finally{d=null,f=a,p=!1}i=void 0}}finally{i?D():S=!1}}}var D;if(typeof y==`function`)D=function(){y(E)};else if(typeof MessageChannel<`u`){var O=new MessageChannel,k=O.port2;O.port1.onmessage=E,D=function(){k.postMessage(null)}}else D=function(){_(E,0)};function A(t,n){C=_(function(){t(e.unstable_now())},n)}e.unstable_IdlePriority=5,e.unstable_ImmediatePriority=1,e.unstable_LowPriority=4,e.unstable_NormalPriority=3,e.unstable_Profiling=null,e.unstable_UserBlockingPriority=2,e.unstable_cancelCallback=function(e){e.callback=null},e.unstable_forceFrameRate=function(e){0>e||125<e?console.error(`forceFrameRate takes a positive int between 0 and 125, forcing frame rates higher than 125 fps is not supported`):w=0<e?Math.floor(1e3/e):5},e.unstable_getCurrentPriorityLevel=function(){return f},e.unstable_next=function(e){switch(f){case 1:case 2:case 3:var t=3;break;default:t=f}var n=f;f=t;try{return e()}finally{f=n}},e.unstable_requestPaint=function(){g=!0},e.unstable_runWithPriority=function(e,t){switch(e){case 1:case 2:case 3:case 4:case 5:break;default:e=3}var n=f;f=e;try{return t()}finally{f=n}},e.unstable_scheduleCallback=function(r,i,a){var o=e.unstable_now();switch(typeof a==`object`&&a?(a=a.delay,a=typeof a==`number`&&0<a?o+a:o):a=o,r){case 1:var s=-1;break;case 2:s=250;break;case 5:s=1073741823;break;case 4:s=1e4;break;default:s=5e3}return s=a+s,r={id:u++,callback:i,priorityLevel:r,startTime:a,expirationTime:s,sortIndex:-1},a>o?(r.sortIndex=a,t(l,r),n(c)===null&&r===n(l)&&(h?(v(C),C=-1):h=!0,A(x,a-o))):(r.sortIndex=s,t(c,r),m||p||(m=!0,S||(S=!0,D()))),r},e.unstable_shouldYield=T,e.unstable_wrapCallback=function(e){var t=f;return function(){var n=f;f=t;try{return e.apply(this,arguments)}finally{f=n}}}})),j=t(((e,t)=>{t.exports=re()})),M=t((e=>{var t=j(),r=n(),i=d();function a(e){var t=`https://react.dev/errors/`+e;if(1<arguments.length){t+=`?args[]=`+encodeURIComponent(arguments[1]);for(var n=2;n<arguments.length;n++)t+=`&args[]=`+encodeURIComponent(arguments[n])}return`Minified React error #`+e+`; visit `+t+` for the full message or use the non-minified dev environment for full errors and additional helpful warnings.`}function o(e){return!(!e||e.nodeType!==1&&e.nodeType!==9&&e.nodeType!==11)}function s(e){var t=e,n=e;if(e.alternate)for(;t.return;)t=t.return;else{e=t;do t=e,t.flags&4098&&(n=t.return),e=t.return;while(e)}return t.tag===3?n:null}function c(e){if(e.tag===13){var t=e.memoizedState;if(t===null&&(e=e.alternate,e!==null&&(t=e.memoizedState)),t!==null)return t.dehydrated}return null}function l(e){if(e.tag===31){var t=e.memoizedState;if(t===null&&(e=e.alternate,e!==null&&(t=e.memoizedState)),t!==null)return t.dehydrated}return null}function u(e){if(s(e)!==e)throw Error(a(188))}function f(e){var t=e.alternate;if(!t){if(t=s(e),t===null)throw Error(a(188));return t===e?e:null}for(var n=e,r=t;;){var i=n.return;if(i===null)break;var o=i.alternate;if(o===null){if(r=i.return,r!==null){n=r;continue}break}if(i.child===o.child){for(o=i.child;o;){if(o===n)return u(i),e;if(o===r)return u(i),t;o=o.sibling}throw Error(a(188))}if(n.return!==r.return)n=i,r=o;else{for(var c=!1,l=i.child;l;){if(l===n){c=!0,n=i,r=o;break}if(l===r){c=!0,r=i,n=o;break}l=l.sibling}if(!c){for(l=o.child;l;){if(l===n){c=!0,n=o,r=i;break}if(l===r){c=!0,r=o,n=i;break}l=l.sibling}if(!c)throw Error(a(189))}}if(n.alternate!==r)throw Error(a(190))}if(n.tag!==3)throw Error(a(188));return n.stateNode.current===n?e:t}function p(e){var t=e.tag;if(t===5||t===26||t===27||t===6)return e;for(e=e.child;e!==null;){if(t=p(e),t!==null)return t;e=e.sibling}return null}var m=Object.assign,h=Symbol.for(`react.element`),g=Symbol.for(`react.transitional.element`),_=Symbol.for(`react.portal`),v=Symbol.for(`react.fragment`),y=Symbol.for(`react.strict_mode`),b=Symbol.for(`react.profiler`),x=Symbol.for(`react.consumer`),S=Symbol.for(`react.context`),C=Symbol.for(`react.forward_ref`),w=Symbol.for(`react.suspense`),ee=Symbol.for(`react.suspense_list`),T=Symbol.for(`react.memo`),E=Symbol.for(`react.lazy`),D=Symbol.for(`react.activity`),O=Symbol.for(`react.memo_cache_sentinel`),k=Symbol.iterator;function A(e){return typeof e!=`object`||!e?null:(e=k&&e[k]||e[`@@iterator`],typeof e==`function`?e:null)}var te=Symbol.for(`react.client.reference`);function ne(e){if(e==null)return null;if(typeof e==`function`)return e.$$typeof===te?null:e.displayName||e.name||null;if(typeof e==`string`)return e;switch(e){case v:return`Fragment`;case b:return`Profiler`;case y:return`StrictMode`;case w:return`Suspense`;case ee:return`SuspenseList`;case D:return`Activity`}if(typeof e==`object`)switch(e.$$typeof){case _:return`Portal`;case S:return e.displayName||`Context`;case x:return(e._context.displayName||`Context`)+`.Consumer`;case C:var t=e.render;return e=e.displayName,e||=(e=t.displayName||t.name||``,e===``?`ForwardRef`:`ForwardRef(`+e+`)`),e;case T:return t=e.displayName||null,t===null?ne(e.type)||`Memo`:t;case E:t=e._payload,e=e._init;try{return ne(e(t))}catch{}}return null}var re=Array.isArray,M=r.__CLIENT_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE,N=i.__DOM_INTERNALS_DO_NOT_USE_OR_WARN_USERS_THEY_CANNOT_UPGRADE,P={pending:!1,data:null,method:null,action:null},ie=[],F=-1;function ae(e){return{current:e}}function I(e){0>F||(e.current=ie[F],ie[F]=null,F--)}function L(e,t){F++,ie[F]=e.current,e.current=t}var oe=ae(null),se=ae(null),ce=ae(null),le=ae(null);function ue(e,t){switch(L(ce,t),L(se,e),L(oe,null),t.nodeType){case 9:case 11:e=(e=t.documentElement)&&(e=e.namespaceURI)?Vd(e):0;break;default:if(e=t.tagName,t=t.namespaceURI)t=Vd(t),e=Hd(t,e);else switch(e){case`svg`:e=1;break;case`math`:e=2;break;default:e=0}}I(oe),L(oe,e)}function R(){I(oe),I(se),I(ce)}function de(e){e.memoizedState!==null&&L(le,e);var t=oe.current,n=Hd(t,e.type);t!==n&&(L(se,e),L(oe,n))}function z(e){se.current===e&&(I(oe),I(se)),le.current===e&&(I(le),Qf._currentValue=P)}var fe,pe;function me(e){if(fe===void 0)try{throw Error()}catch(e){var t=e.stack.trim().match(/\n( *(at )?)/);fe=t&&t[1]||``,pe=-1<e.stack.indexOf(`
    at`)?` (<anonymous>)`:-1<e.stack.indexOf(`@`)?`@unknown:0:0`:``}return`
`+fe+e+pe}var he=!1;function ge(e,t){if(!e||he)return``;he=!0;var n=Error.prepareStackTrace;Error.prepareStackTrace=void 0;try{var r={DetermineComponentFrameRoot:function(){try{if(t){var n=function(){throw Error()};if(Object.defineProperty(n.prototype,"props",{set:function(){throw Error()}}),typeof Reflect==`object`&&Reflect.construct){try{Reflect.construct(n,[])}catch(e){var r=e}Reflect.construct(e,[],n)}else{try{n.call()}catch(e){r=e}e.call(n.prototype)}}else{try{throw Error()}catch(e){r=e}(n=e())&&typeof n.catch==`function`&&n.catch(function(){})}}catch(e){if(e&&r&&typeof e.stack==`string`)return[e.stack,r.stack]}return[null,null]}};r.DetermineComponentFrameRoot.displayName=`DetermineComponentFrameRoot`;var i=Object.getOwnPropertyDescriptor(r.DetermineComponentFrameRoot,`name`);i&&i.configurable&&Object.defineProperty(r.DetermineComponentFrameRoot,"name",{value:`DetermineComponentFrameRoot`});var a=r.DetermineComponentFrameRoot(),o=a[0],s=a[1];if(o&&s){var c=o.split(`
`),l=s.split(`
`);for(i=r=0;r<c.length&&!c[r].includes(`DetermineComponentFrameRoot`);)r++;for(;i<l.length&&!l[i].includes(`DetermineComponentFrameRoot`);)i++;if(r===c.length||i===l.length)for(r=c.length-1,i=l.length-1;1<=r&&0<=i&&c[r]!==l[i];)i--;for(;1<=r&&0<=i;r--,i--)if(c[r]!==l[i]){if(r!==1||i!==1)do if(r--,i--,0>i||c[r]!==l[i]){var u=`
`+c[r].replace(` at new `,` at `);return e.displayName&&u.includes(`<anonymous>`)&&(u=u.replace(`<anonymous>`,e.displayName)),u}while(1<=r&&0<=i);break}}}finally{he=!1,Error.prepareStackTrace=n}return(n=e?e.displayName||e.name:``)?me(n):``}function _e(e,t){switch(e.tag){case 26:case 27:case 5:return me(e.type);case 16:return me(`Lazy`);case 13:return e.child!==t&&t!==null?me(`Suspense Fallback`):me(`Suspense`);case 19:return me(`SuspenseList`);case 0:case 15:return ge(e.type,!1);case 11:return ge(e.type.render,!1);case 1:return ge(e.type,!0);case 31:return me(`Activity`);default:return``}}function ve(e){try{var t=``,n=null;do t+=_e(e,n),n=e,e=e.return;while(e);return t}catch(e){return`
Error generating stack: `+e.message+`
`+e.stack}}var ye=Object.prototype.hasOwnProperty,be=t.unstable_scheduleCallback,xe=t.unstable_cancelCallback,Se=t.unstable_shouldYield,Ce=t.unstable_requestPaint,we=t.unstable_now,Te=t.unstable_getCurrentPriorityLevel,Ee=t.unstable_ImmediatePriority,De=t.unstable_UserBlockingPriority,Oe=t.unstable_NormalPriority,ke=t.unstable_LowPriority,Ae=t.unstable_IdlePriority,je=t.log,Me=t.unstable_setDisableYieldValue,Ne=null,Pe=null;function Fe(e){if(typeof je==`function`&&Me(e),Pe&&typeof Pe.setStrictMode==`function`)try{Pe.setStrictMode(Ne,e)}catch{}}var Ie=Math.clz32?Math.clz32:ze,Le=Math.log,Re=Math.LN2;function ze(e){return e>>>=0,e===0?32:31-(Le(e)/Re|0)|0}var Be=256,Ve=262144,He=4194304;function Ue(e){var t=e&42;if(t!==0)return t;switch(e&-e){case 1:return 1;case 2:return 2;case 4:return 4;case 8:return 8;case 16:return 16;case 32:return 32;case 64:return 64;case 128:return 128;case 256:case 512:case 1024:case 2048:case 4096:case 8192:case 16384:case 32768:case 65536:case 131072:return e&261888;case 262144:case 524288:case 1048576:case 2097152:return e&3932160;case 4194304:case 8388608:case 16777216:case 33554432:return e&62914560;case 67108864:return 67108864;case 134217728:return 134217728;case 268435456:return 268435456;case 536870912:return 536870912;case 1073741824:return 0;default:return e}}function We(e,t,n){var r=e.pendingLanes;if(r===0)return 0;var i=0,a=e.suspendedLanes,o=e.pingedLanes;e=e.warmLanes;var s=r&134217727;return s===0?(s=r&~a,s===0?o===0?n||(n=r&~e,n!==0&&(i=Ue(n))):i=Ue(o):i=Ue(s)):(r=s&~a,r===0?(o&=s,o===0?n||(n=s&~e,n!==0&&(i=Ue(n))):i=Ue(o)):i=Ue(r)),i===0?0:t!==0&&t!==i&&(t&a)===0&&(a=i&-i,n=t&-t,a>=n||a===32&&n&4194048)?t:i}function Ge(e,t){return(e.pendingLanes&~(e.suspendedLanes&~e.pingedLanes)&t)===0}function Ke(e,t){switch(e){case 1:case 2:case 4:case 8:case 64:return t+250;case 16:case 32:case 128:case 256:case 512:case 1024:case 2048:case 4096:case 8192:case 16384:case 32768:case 65536:case 131072:case 262144:case 524288:case 1048576:case 2097152:return t+5e3;case 4194304:case 8388608:case 16777216:case 33554432:return-1;case 67108864:case 134217728:case 268435456:case 536870912:case 1073741824:return-1;default:return-1}}function qe(){var e=He;return He<<=1,!(He&62914560)&&(He=4194304),e}function Je(e){for(var t=[],n=0;31>n;n++)t.push(e);return t}function Ye(e,t){e.pendingLanes|=t,t!==268435456&&(e.suspendedLanes=0,e.pingedLanes=0,e.warmLanes=0)}function Xe(e,t,n,r,i,a){var o=e.pendingLanes;e.pendingLanes=n,e.suspendedLanes=0,e.pingedLanes=0,e.warmLanes=0,e.expiredLanes&=n,e.entangledLanes&=n,e.errorRecoveryDisabledLanes&=n,e.shellSuspendCounter=0;var s=e.entanglements,c=e.expirationTimes,l=e.hiddenUpdates;for(n=o&~n;0<n;){var u=31-Ie(n),d=1<<u;s[u]=0,c[u]=-1;var f=l[u];if(f!==null)for(l[u]=null,u=0;u<f.length;u++){var p=f[u];p!==null&&(p.lane&=-536870913)}n&=~d}r!==0&&Ze(e,r,0),a!==0&&i===0&&e.tag!==0&&(e.suspendedLanes|=a&~(o&~t))}function Ze(e,t,n){e.pendingLanes|=t,e.suspendedLanes&=~t;var r=31-Ie(t);e.entangledLanes|=t,e.entanglements[r]=e.entanglements[r]|1073741824|n&261930}function Qe(e,t){var n=e.entangledLanes|=t;for(e=e.entanglements;n;){var r=31-Ie(n),i=1<<r;i&t|e[r]&t&&(e[r]|=t),n&=~i}}function $e(e,t){var n=t&-t;return n=n&42?1:et(n),(n&(e.suspendedLanes|t))===0?n:0}function et(e){switch(e){case 2:e=1;break;case 8:e=4;break;case 32:e=16;break;case 256:case 512:case 1024:case 2048:case 4096:case 8192:case 16384:case 32768:case 65536:case 131072:case 262144:case 524288:case 1048576:case 2097152:case 4194304:case 8388608:case 16777216:case 33554432:e=128;break;case 268435456:e=134217728;break;default:e=0}return e}function tt(e){return e&=-e,2<e?8<e?e&134217727?32:268435456:8:2}function nt(){var e=N.p;return e===0?(e=window.event,e===void 0?32:mp(e.type)):e}function rt(e,t){var n=N.p;try{return N.p=e,t()}finally{N.p=n}}var it=Math.random().toString(36).slice(2),at=`__reactFiber$`+it,ot=`__reactProps$`+it,st=`__reactContainer$`+it,ct=`__reactEvents$`+it,lt=`__reactListeners$`+it,ut=`__reactHandles$`+it,dt=`__reactResources$`+it,ft=`__reactMarker$`+it;function pt(e){delete e[at],delete e[ot],delete e[ct],delete e[lt],delete e[ut]}function mt(e){var t=e[at];if(t)return t;for(var n=e.parentNode;n;){if(t=n[st]||n[at]){if(n=t.alternate,t.child!==null||n!==null&&n.child!==null)for(e=df(e);e!==null;){if(n=e[at])return n;e=df(e)}return t}e=n,n=e.parentNode}return null}function ht(e){if(e=e[at]||e[st]){var t=e.tag;if(t===5||t===6||t===13||t===31||t===26||t===27||t===3)return e}return null}function gt(e){var t=e.tag;if(t===5||t===26||t===27||t===6)return e.stateNode;throw Error(a(33))}function _t(e){var t=e[dt];return t||=e[dt]={hoistableStyles:new Map,hoistableScripts:new Map},t}function vt(e){e[ft]=!0}var yt=new Set,bt={};function xt(e,t){St(e,t),St(e+`Capture`,t)}function St(e,t){for(bt[e]=t,e=0;e<t.length;e++)yt.add(t[e])}var Ct=RegExp(`^[:A-Z_a-z\\u00C0-\\u00D6\\u00D8-\\u00F6\\u00F8-\\u02FF\\u0370-\\u037D\\u037F-\\u1FFF\\u200C-\\u200D\\u2070-\\u218F\\u2C00-\\u2FEF\\u3001-\\uD7FF\\uF900-\\uFDCF\\uFDF0-\\uFFFD][:A-Z_a-z\\u00C0-\\u00D6\\u00D8-\\u00F6\\u00F8-\\u02FF\\u0370-\\u037D\\u037F-\\u1FFF\\u200C-\\u200D\\u2070-\\u218F\\u2C00-\\u2FEF\\u3001-\\uD7FF\\uF900-\\uFDCF\\uFDF0-\\uFFFD\\-.0-9\\u00B7\\u0300-\\u036F\\u203F-\\u2040]*$`),wt={},Tt={};function Et(e){return ye.call(Tt,e)?!0:ye.call(wt,e)?!1:Ct.test(e)?Tt[e]=!0:(wt[e]=!0,!1)}function Dt(e,t,n){if(Et(t))if(n===null)e.removeAttribute(t);else{switch(typeof n){case`undefined`:case`function`:case`symbol`:e.removeAttribute(t);return;case`boolean`:var r=t.toLowerCase().slice(0,5);if(r!==`data-`&&r!==`aria-`){e.removeAttribute(t);return}}e.setAttribute(t,``+n)}}function Ot(e,t,n){if(n===null)e.removeAttribute(t);else{switch(typeof n){case`undefined`:case`function`:case`symbol`:case`boolean`:e.removeAttribute(t);return}e.setAttribute(t,``+n)}}function kt(e,t,n,r){if(r===null)e.removeAttribute(n);else{switch(typeof r){case`undefined`:case`function`:case`symbol`:case`boolean`:e.removeAttribute(n);return}e.setAttributeNS(t,n,``+r)}}function At(e){switch(typeof e){case`bigint`:case`boolean`:case`number`:case`string`:case`undefined`:return e;case`object`:return e;default:return``}}function jt(e){var t=e.type;return(e=e.nodeName)&&e.toLowerCase()===`input`&&(t===`checkbox`||t===`radio`)}function Mt(e,t,n){var r=Object.getOwnPropertyDescriptor(e.constructor.prototype,t);if(!e.hasOwnProperty(t)&&r!==void 0&&typeof r.get==`function`&&typeof r.set==`function`){var i=r.get,a=r.set;return Object.defineProperty(e,t,{configurable:!0,get:function(){return i.call(this)},set:function(e){n=``+e,a.call(this,e)}}),Object.defineProperty(e,t,{enumerable:r.enumerable}),{getValue:function(){return n},setValue:function(e){n=``+e},stopTracking:function(){e._valueTracker=null,delete e[t]}}}}function Nt(e){if(!e._valueTracker){var t=jt(e)?`checked`:`value`;e._valueTracker=Mt(e,t,``+e[t])}}function Pt(e){if(!e)return!1;var t=e._valueTracker;if(!t)return!0;var n=t.getValue(),r=``;return e&&(r=jt(e)?e.checked?`true`:`false`:e.value),e=r,e===n?!1:(t.setValue(e),!0)}function Ft(e){if(e||=typeof document<`u`?document:void 0,e===void 0)return null;try{return e.activeElement||e.body}catch{return e.body}}var It=/[\n"\\]/g;function Lt(e){return e.replace(It,function(e){return`\\`+e.charCodeAt(0).toString(16)+` `})}function Rt(e,t,n,r,i,a,o,s){e.name=``,o!=null&&typeof o!=`function`&&typeof o!=`symbol`&&typeof o!=`boolean`?e.type=o:e.removeAttribute(`type`),t==null?o!==`submit`&&o!==`reset`||e.removeAttribute(`value`):o===`number`?(t===0&&e.value===``||e.value!=t)&&(e.value=``+At(t)):e.value!==``+At(t)&&(e.value=``+At(t)),t==null?n==null?r!=null&&e.removeAttribute(`value`):Bt(e,o,At(n)):Bt(e,o,At(t)),i==null&&a!=null&&(e.defaultChecked=!!a),i!=null&&(e.checked=i&&typeof i!=`function`&&typeof i!=`symbol`),s!=null&&typeof s!=`function`&&typeof s!=`symbol`&&typeof s!=`boolean`?e.name=``+At(s):e.removeAttribute(`name`)}function zt(e,t,n,r,i,a,o,s){if(a!=null&&typeof a!=`function`&&typeof a!=`symbol`&&typeof a!=`boolean`&&(e.type=a),t!=null||n!=null){if(!(a!==`submit`&&a!==`reset`||t!=null)){Nt(e);return}n=n==null?``:``+At(n),t=t==null?n:``+At(t),s||t===e.value||(e.value=t),e.defaultValue=t}r??=i,r=typeof r!=`function`&&typeof r!=`symbol`&&!!r,e.checked=s?e.checked:!!r,e.defaultChecked=!!r,o!=null&&typeof o!=`function`&&typeof o!=`symbol`&&typeof o!=`boolean`&&(e.name=o),Nt(e)}function Bt(e,t,n){t===`number`&&Ft(e.ownerDocument)===e||e.defaultValue===``+n||(e.defaultValue=``+n)}function Vt(e,t,n,r){if(e=e.options,t){t={};for(var i=0;i<n.length;i++)t[`$`+n[i]]=!0;for(n=0;n<e.length;n++)i=t.hasOwnProperty(`$`+e[n].value),e[n].selected!==i&&(e[n].selected=i),i&&r&&(e[n].defaultSelected=!0)}else{for(n=``+At(n),t=null,i=0;i<e.length;i++){if(e[i].value===n){e[i].selected=!0,r&&(e[i].defaultSelected=!0);return}t!==null||e[i].disabled||(t=e[i])}t!==null&&(t.selected=!0)}}function Ht(e,t,n){if(t!=null&&(t=``+At(t),t!==e.value&&(e.value=t),n==null)){e.defaultValue!==t&&(e.defaultValue=t);return}e.defaultValue=n==null?``:``+At(n)}function Ut(e,t,n,r){if(t==null){if(r!=null){if(n!=null)throw Error(a(92));if(re(r)){if(1<r.length)throw Error(a(93));r=r[0]}n=r}n??=``,t=n}n=At(t),e.defaultValue=n,r=e.textContent,r===n&&r!==``&&r!==null&&(e.value=r),Nt(e)}function Wt(e,t){if(t){var n=e.firstChild;if(n&&n===e.lastChild&&n.nodeType===3){n.nodeValue=t;return}}e.textContent=t}var Gt=new Set(`animationIterationCount aspectRatio borderImageOutset borderImageSlice borderImageWidth boxFlex boxFlexGroup boxOrdinalGroup columnCount columns flex flexGrow flexPositive flexShrink flexNegative flexOrder gridArea gridRow gridRowEnd gridRowSpan gridRowStart gridColumn gridColumnEnd gridColumnSpan gridColumnStart fontWeight lineClamp lineHeight opacity order orphans scale tabSize widows zIndex zoom fillOpacity floodOpacity stopOpacity strokeDasharray strokeDashoffset strokeMiterlimit strokeOpacity strokeWidth MozAnimationIterationCount MozBoxFlex MozBoxFlexGroup MozLineClamp msAnimationIterationCount msFlex msZoom msFlexGrow msFlexNegative msFlexOrder msFlexPositive msFlexShrink msGridColumn msGridColumnSpan msGridRow msGridRowSpan WebkitAnimationIterationCount WebkitBoxFlex WebKitBoxFlexGroup WebkitBoxOrdinalGroup WebkitColumnCount WebkitColumns WebkitFlex WebkitFlexGrow WebkitFlexPositive WebkitFlexShrink WebkitLineClamp`.split(` `));function Kt(e,t,n){var r=t.indexOf(`--`)===0;n==null||typeof n==`boolean`||n===``?r?e.setProperty(t,``):t===`float`?e.cssFloat=``:e[t]=``:r?e.setProperty(t,n):typeof n!=`number`||n===0||Gt.has(t)?t===`float`?e.cssFloat=n:e[t]=(``+n).trim():e[t]=n+`px`}function qt(e,t,n){if(t!=null&&typeof t!=`object`)throw Error(a(62));if(e=e.style,n!=null){for(var r in n)!n.hasOwnProperty(r)||t!=null&&t.hasOwnProperty(r)||(r.indexOf(`--`)===0?e.setProperty(r,``):r===`float`?e.cssFloat=``:e[r]=``);for(var i in t)r=t[i],t.hasOwnProperty(i)&&n[i]!==r&&Kt(e,i,r)}else for(var o in t)t.hasOwnProperty(o)&&Kt(e,o,t[o])}function Jt(e){if(e.indexOf(`-`)===-1)return!1;switch(e){case`annotation-xml`:case`color-profile`:case`font-face`:case`font-face-src`:case`font-face-uri`:case`font-face-format`:case`font-face-name`:case`missing-glyph`:return!1;default:return!0}}var Yt=new Map([[`acceptCharset`,`accept-charset`],[`htmlFor`,`for`],[`httpEquiv`,`http-equiv`],[`crossOrigin`,`crossorigin`],[`accentHeight`,`accent-height`],[`alignmentBaseline`,`alignment-baseline`],[`arabicForm`,`arabic-form`],[`baselineShift`,`baseline-shift`],[`capHeight`,`cap-height`],[`clipPath`,`clip-path`],[`clipRule`,`clip-rule`],[`colorInterpolation`,`color-interpolation`],[`colorInterpolationFilters`,`color-interpolation-filters`],[`colorProfile`,`color-profile`],[`colorRendering`,`color-rendering`],[`dominantBaseline`,`dominant-baseline`],[`enableBackground`,`enable-background`],[`fillOpacity`,`fill-opacity`],[`fillRule`,`fill-rule`],[`floodColor`,`flood-color`],[`floodOpacity`,`flood-opacity`],[`fontFamily`,`font-family`],[`fontSize`,`font-size`],[`fontSizeAdjust`,`font-size-adjust`],[`fontStretch`,`font-stretch`],[`fontStyle`,`font-style`],[`fontVariant`,`font-variant`],[`fontWeight`,`font-weight`],[`glyphName`,`glyph-name`],[`glyphOrientationHorizontal`,`glyph-orientation-horizontal`],[`glyphOrientationVertical`,`glyph-orientation-vertical`],[`horizAdvX`,`horiz-adv-x`],[`horizOriginX`,`horiz-origin-x`],[`imageRendering`,`image-rendering`],[`letterSpacing`,`letter-spacing`],[`lightingColor`,`lighting-color`],[`markerEnd`,`marker-end`],[`markerMid`,`marker-mid`],[`markerStart`,`marker-start`],[`overlinePosition`,`overline-position`],[`overlineThickness`,`overline-thickness`],[`paintOrder`,`paint-order`],[`panose-1`,`panose-1`],[`pointerEvents`,`pointer-events`],[`renderingIntent`,`rendering-intent`],[`shapeRendering`,`shape-rendering`],[`stopColor`,`stop-color`],[`stopOpacity`,`stop-opacity`],[`strikethroughPosition`,`strikethrough-position`],[`strikethroughThickness`,`strikethrough-thickness`],[`strokeDasharray`,`stroke-dasharray`],[`strokeDashoffset`,`stroke-dashoffset`],[`strokeLinecap`,`stroke-linecap`],[`strokeLinejoin`,`stroke-linejoin`],[`strokeMiterlimit`,`stroke-miterlimit`],[`strokeOpacity`,`stroke-opacity`],[`strokeWidth`,`stroke-width`],[`textAnchor`,`text-anchor`],[`textDecoration`,`text-decoration`],[`textRendering`,`text-rendering`],[`transformOrigin`,`transform-origin`],[`underlinePosition`,`underline-position`],[`underlineThickness`,`underline-thickness`],[`unicodeBidi`,`unicode-bidi`],[`unicodeRange`,`unicode-range`],[`unitsPerEm`,`units-per-em`],[`vAlphabetic`,`v-alphabetic`],[`vHanging`,`v-hanging`],[`vIdeographic`,`v-ideographic`],[`vMathematical`,`v-mathematical`],[`vectorEffect`,`vector-effect`],[`vertAdvY`,`vert-adv-y`],[`vertOriginX`,`vert-origin-x`],[`vertOriginY`,`vert-origin-y`],[`wordSpacing`,`word-spacing`],[`writingMode`,`writing-mode`],[`xmlnsXlink`,`xmlns:xlink`],[`xHeight`,`x-height`]]),Xt=/^[\u0000-\u001F ]*j[\r\n\t]*a[\r\n\t]*v[\r\n\t]*a[\r\n\t]*s[\r\n\t]*c[\r\n\t]*r[\r\n\t]*i[\r\n\t]*p[\r\n\t]*t[\r\n\t]*:/i;function Zt(e){return Xt.test(``+e)?`javascript:throw new Error('React has blocked a javascript: URL as a security precaution.')`:e}function Qt(){}var $t=null;function en(e){return e=e.target||e.srcElement||window,e.correspondingUseElement&&(e=e.correspondingUseElement),e.nodeType===3?e.parentNode:e}var tn=null,nn=null;function rn(e){var t=ht(e);if(t&&(e=t.stateNode)){var n=e[ot]||null;a:switch(e=t.stateNode,t.type){case`input`:if(Rt(e,n.value,n.defaultValue,n.defaultValue,n.checked,n.defaultChecked,n.type,n.name),t=n.name,n.type===`radio`&&t!=null){for(n=e;n.parentNode;)n=n.parentNode;for(n=n.querySelectorAll(`input[name="`+Lt(``+t)+`"][type="radio"]`),t=0;t<n.length;t++){var r=n[t];if(r!==e&&r.form===e.form){var i=r[ot]||null;if(!i)throw Error(a(90));Rt(r,i.value,i.defaultValue,i.defaultValue,i.checked,i.defaultChecked,i.type,i.name)}}for(t=0;t<n.length;t++)r=n[t],r.form===e.form&&Pt(r)}break a;case`textarea`:Ht(e,n.value,n.defaultValue);break a;case`select`:t=n.value,t!=null&&Vt(e,!!n.multiple,t,!1)}}}var an=!1;function on(e,t,n){if(an)return e(t,n);an=!0;try{return e(t)}finally{if(an=!1,(tn!==null||nn!==null)&&(bu(),tn&&(t=tn,e=nn,nn=tn=null,rn(t),e)))for(t=0;t<e.length;t++)rn(e[t])}}function sn(e,t){var n=e.stateNode;if(n===null)return null;var r=n[ot]||null;if(r===null)return null;n=r[t];a:switch(t){case`onClick`:case`onClickCapture`:case`onDoubleClick`:case`onDoubleClickCapture`:case`onMouseDown`:case`onMouseDownCapture`:case`onMouseMove`:case`onMouseMoveCapture`:case`onMouseUp`:case`onMouseUpCapture`:case`onMouseEnter`:(r=!r.disabled)||(e=e.type,r=!(e===`button`||e===`input`||e===`select`||e===`textarea`)),e=!r;break a;default:e=!1}if(e)return null;if(n&&typeof n!=`function`)throw Error(a(231,t,typeof n));return n}var cn=!(typeof window>`u`||window.document===void 0||window.document.createElement===void 0),ln=!1;if(cn)try{var un={};Object.defineProperty(un,"passive",{get:function(){ln=!0}}),window.addEventListener(`test`,un,un),window.removeEventListener(`test`,un,un)}catch{ln=!1}var dn=null,fn=null,pn=null;function mn(){if(pn)return pn;var e,t=fn,n=t.length,r,i=`value`in dn?dn.value:dn.textContent,a=i.length;for(e=0;e<n&&t[e]===i[e];e++);var o=n-e;for(r=1;r<=o&&t[n-r]===i[a-r];r++);return pn=i.slice(e,1<r?1-r:void 0)}function hn(e){var t=e.keyCode;return`charCode`in e?(e=e.charCode,e===0&&t===13&&(e=13)):e=t,e===10&&(e=13),32<=e||e===13?e:0}function gn(){return!0}function _n(){return!1}function vn(e){function t(t,n,r,i,a){for(var o in this._reactName=t,this._targetInst=r,this.type=n,this.nativeEvent=i,this.target=a,this.currentTarget=null,e)e.hasOwnProperty(o)&&(t=e[o],this[o]=t?t(i):i[o]);return this.isDefaultPrevented=(i.defaultPrevented==null?!1===i.returnValue:i.defaultPrevented)?gn:_n,this.isPropagationStopped=_n,this}return m(t.prototype,{preventDefault:function(){this.defaultPrevented=!0;var e=this.nativeEvent;e&&(e.preventDefault?e.preventDefault():typeof e.returnValue!=`unknown`&&(e.returnValue=!1),this.isDefaultPrevented=gn)},stopPropagation:function(){var e=this.nativeEvent;e&&(e.stopPropagation?e.stopPropagation():typeof e.cancelBubble!=`unknown`&&(e.cancelBubble=!0),this.isPropagationStopped=gn)},persist:function(){},isPersistent:gn}),t}var yn={eventPhase:0,bubbles:0,cancelable:0,timeStamp:function(e){return e.timeStamp||Date.now()},defaultPrevented:0,isTrusted:0},bn=vn(yn),xn=m({},yn,{view:0,detail:0}),Sn=vn(xn),Cn,wn,Tn,En=m({},xn,{screenX:0,screenY:0,clientX:0,clientY:0,pageX:0,pageY:0,ctrlKey:0,shiftKey:0,altKey:0,metaKey:0,getModifierState:Ln,button:0,buttons:0,relatedTarget:function(e){return e.relatedTarget===void 0?e.fromElement===e.srcElement?e.toElement:e.fromElement:e.relatedTarget},movementX:function(e){return`movementX`in e?e.movementX:(e!==Tn&&(Tn&&e.type===`mousemove`?(Cn=e.screenX-Tn.screenX,wn=e.screenY-Tn.screenY):wn=Cn=0,Tn=e),Cn)},movementY:function(e){return`movementY`in e?e.movementY:wn}}),Dn=vn(En),On=vn(m({},En,{dataTransfer:0})),kn=vn(m({},xn,{relatedTarget:0})),An=vn(m({},yn,{animationName:0,elapsedTime:0,pseudoElement:0})),jn=vn(m({},yn,{clipboardData:function(e){return`clipboardData`in e?e.clipboardData:window.clipboardData}})),Mn=vn(m({},yn,{data:0})),Nn={Esc:`Escape`,Spacebar:` `,Left:`ArrowLeft`,Up:`ArrowUp`,Right:`ArrowRight`,Down:`ArrowDown`,Del:`Delete`,Win:`OS`,Menu:`ContextMenu`,Apps:`ContextMenu`,Scroll:`ScrollLock`,MozPrintableKey:`Unidentified`},Pn={8:`Backspace`,9:`Tab`,12:`Clear`,13:`Enter`,16:`Shift`,17:`Control`,18:`Alt`,19:`Pause`,20:`CapsLock`,27:`Escape`,32:` `,33:`PageUp`,34:`PageDown`,35:`End`,36:`Home`,37:`ArrowLeft`,38:`ArrowUp`,39:`ArrowRight`,40:`ArrowDown`,45:`Insert`,46:`Delete`,112:`F1`,113:`F2`,114:`F3`,115:`F4`,116:`F5`,117:`F6`,118:`F7`,119:`F8`,120:`F9`,121:`F10`,122:`F11`,123:`F12`,144:`NumLock`,145:`ScrollLock`,224:`Meta`},Fn={Alt:`altKey`,Control:`ctrlKey`,Meta:`metaKey`,Shift:`shiftKey`};function In(e){var t=this.nativeEvent;return t.getModifierState?t.getModifierState(e):(e=Fn[e])?!!t[e]:!1}function Ln(){return In}var Rn=vn(m({},xn,{key:function(e){if(e.key){var t=Nn[e.key]||e.key;if(t!==`Unidentified`)return t}return e.type===`keypress`?(e=hn(e),e===13?`Enter`:String.fromCharCode(e)):e.type===`keydown`||e.type===`keyup`?Pn[e.keyCode]||`Unidentified`:``},code:0,location:0,ctrlKey:0,shiftKey:0,altKey:0,metaKey:0,repeat:0,locale:0,getModifierState:Ln,charCode:function(e){return e.type===`keypress`?hn(e):0},keyCode:function(e){return e.type===`keydown`||e.type===`keyup`?e.keyCode:0},which:function(e){return e.type===`keypress`?hn(e):e.type===`keydown`||e.type===`keyup`?e.keyCode:0}})),zn=vn(m({},En,{pointerId:0,width:0,height:0,pressure:0,tangentialPressure:0,tiltX:0,tiltY:0,twist:0,pointerType:0,isPrimary:0})),Bn=vn(m({},xn,{touches:0,targetTouches:0,changedTouches:0,altKey:0,metaKey:0,ctrlKey:0,shiftKey:0,getModifierState:Ln})),Vn=vn(m({},yn,{propertyName:0,elapsedTime:0,pseudoElement:0})),Hn=vn(m({},En,{deltaX:function(e){return`deltaX`in e?e.deltaX:`wheelDeltaX`in e?-e.wheelDeltaX:0},deltaY:function(e){return`deltaY`in e?e.deltaY:`wheelDeltaY`in e?-e.wheelDeltaY:`wheelDelta`in e?-e.wheelDelta:0},deltaZ:0,deltaMode:0})),Un=vn(m({},yn,{newState:0,oldState:0})),Wn=[9,13,27,32],Gn=cn&&`CompositionEvent`in window,Kn=null;cn&&`documentMode`in document&&(Kn=document.documentMode);var qn=cn&&`TextEvent`in window&&!Kn,Jn=cn&&(!Gn||Kn&&8<Kn&&11>=Kn),Yn=` `,Xn=!1;function Zn(e,t){switch(e){case`keyup`:return Wn.indexOf(t.keyCode)!==-1;case`keydown`:return t.keyCode!==229;case`keypress`:case`mousedown`:case`focusout`:return!0;default:return!1}}function Qn(e){return e=e.detail,typeof e==`object`&&`data`in e?e.data:null}var $n=!1;function er(e,t){switch(e){case`compositionend`:return Qn(t);case`keypress`:return t.which===32?(Xn=!0,Yn):null;case`textInput`:return e=t.data,e===Yn&&Xn?null:e;default:return null}}function tr(e,t){if($n)return e===`compositionend`||!Gn&&Zn(e,t)?(e=mn(),pn=fn=dn=null,$n=!1,e):null;switch(e){case`paste`:return null;case`keypress`:if(!(t.ctrlKey||t.altKey||t.metaKey)||t.ctrlKey&&t.altKey){if(t.char&&1<t.char.length)return t.char;if(t.which)return String.fromCharCode(t.which)}return null;case`compositionend`:return Jn&&t.locale!==`ko`?null:t.data;default:return null}}var nr={color:!0,date:!0,datetime:!0,"datetime-local":!0,email:!0,month:!0,number:!0,password:!0,range:!0,search:!0,tel:!0,text:!0,time:!0,url:!0,week:!0};function rr(e){var t=e&&e.nodeName&&e.nodeName.toLowerCase();return t===`input`?!!nr[e.type]:t===`textarea`}function ir(e,t,n,r){tn?nn?nn.push(r):nn=[r]:tn=r,t=Td(t,`onChange`),0<t.length&&(n=new bn(`onChange`,`change`,null,n,r),e.push({event:n,listeners:t}))}var ar=null,or=null;function sr(e){vd(e,0)}function cr(e){if(Pt(gt(e)))return e}function lr(e,t){if(e===`change`)return t}var ur=!1;if(cn){var dr;if(cn){var fr=`oninput`in document;if(!fr){var pr=document.createElement(`div`);pr.setAttribute(`oninput`,`return;`),fr=typeof pr.oninput==`function`}dr=fr}else dr=!1;ur=dr&&(!document.documentMode||9<document.documentMode)}function mr(){ar&&(ar.detachEvent(`onpropertychange`,hr),or=ar=null)}function hr(e){if(e.propertyName===`value`&&cr(or)){var t=[];ir(t,or,e,en(e)),on(sr,t)}}function gr(e,t,n){e===`focusin`?(mr(),ar=t,or=n,ar.attachEvent(`onpropertychange`,hr)):e===`focusout`&&mr()}function _r(e){if(e===`selectionchange`||e===`keyup`||e===`keydown`)return cr(or)}function vr(e,t){if(e===`click`)return cr(t)}function yr(e,t){if(e===`input`||e===`change`)return cr(t)}function br(e,t){return e===t&&(e!==0||1/e==1/t)||e!==e&&t!==t}var xr=typeof Object.is==`function`?Object.is:br;function Sr(e,t){if(xr(e,t))return!0;if(typeof e!=`object`||!e||typeof t!=`object`||!t)return!1;var n=Object.keys(e),r=Object.keys(t);if(n.length!==r.length)return!1;for(r=0;r<n.length;r++){var i=n[r];if(!ye.call(t,i)||!xr(e[i],t[i]))return!1}return!0}function Cr(e){for(;e&&e.firstChild;)e=e.firstChild;return e}function wr(e,t){var n=Cr(e);e=0;for(var r;n;){if(n.nodeType===3){if(r=e+n.textContent.length,e<=t&&r>=t)return{node:n,offset:t-e};e=r}a:{for(;n;){if(n.nextSibling){n=n.nextSibling;break a}n=n.parentNode}n=void 0}n=Cr(n)}}function Tr(e,t){return e&&t?e===t?!0:e&&e.nodeType===3?!1:t&&t.nodeType===3?Tr(e,t.parentNode):`contains`in e?e.contains(t):e.compareDocumentPosition?!!(e.compareDocumentPosition(t)&16):!1:!1}function Er(e){e=e!=null&&e.ownerDocument!=null&&e.ownerDocument.defaultView!=null?e.ownerDocument.defaultView:window;for(var t=Ft(e.document);t instanceof e.HTMLIFrameElement;){try{var n=typeof t.contentWindow.location.href==`string`}catch{n=!1}if(n)e=t.contentWindow;else break;t=Ft(e.document)}return t}function Dr(e){var t=e&&e.nodeName&&e.nodeName.toLowerCase();return t&&(t===`input`&&(e.type===`text`||e.type===`search`||e.type===`tel`||e.type===`url`||e.type===`password`)||t===`textarea`||e.contentEditable===`true`)}var Or=cn&&`documentMode`in document&&11>=document.documentMode,kr=null,Ar=null,jr=null,Mr=!1;function Nr(e,t,n){var r=n.window===n?n.document:n.nodeType===9?n:n.ownerDocument;Mr||kr==null||kr!==Ft(r)||(r=kr,`selectionStart`in r&&Dr(r)?r={start:r.selectionStart,end:r.selectionEnd}:(r=(r.ownerDocument&&r.ownerDocument.defaultView||window).getSelection(),r={anchorNode:r.anchorNode,anchorOffset:r.anchorOffset,focusNode:r.focusNode,focusOffset:r.focusOffset}),jr&&Sr(jr,r)||(jr=r,r=Td(Ar,`onSelect`),0<r.length&&(t=new bn(`onSelect`,`select`,null,t,n),e.push({event:t,listeners:r}),t.target=kr)))}function Pr(e,t){var n={};return n[e.toLowerCase()]=t.toLowerCase(),n[`Webkit`+e]=`webkit`+t,n[`Moz`+e]=`moz`+t,n}var Fr={animationend:Pr(`Animation`,`AnimationEnd`),animationiteration:Pr(`Animation`,`AnimationIteration`),animationstart:Pr(`Animation`,`AnimationStart`),transitionrun:Pr(`Transition`,`TransitionRun`),transitionstart:Pr(`Transition`,`TransitionStart`),transitioncancel:Pr(`Transition`,`TransitionCancel`),transitionend:Pr(`Transition`,`TransitionEnd`)},Ir={},Lr={};cn&&(Lr=document.createElement(`div`).style,`AnimationEvent`in window||(delete Fr.animationend.animation,delete Fr.animationiteration.animation,delete Fr.animationstart.animation),`TransitionEvent`in window||delete Fr.transitionend.transition);function Rr(e){if(Ir[e])return Ir[e];if(!Fr[e])return e;var t=Fr[e],n;for(n in t)if(t.hasOwnProperty(n)&&n in Lr)return Ir[e]=t[n];return e}var zr=Rr(`animationend`),Br=Rr(`animationiteration`),Vr=Rr(`animationstart`),Hr=Rr(`transitionrun`),B=Rr(`transitionstart`),Ur=Rr(`transitioncancel`),Wr=Rr(`transitionend`),Gr=new Map,Kr=`abort auxClick beforeToggle cancel canPlay canPlayThrough click close contextMenu copy cut drag dragEnd dragEnter dragExit dragLeave dragOver dragStart drop durationChange emptied encrypted ended error gotPointerCapture input invalid keyDown keyPress keyUp load loadedData loadedMetadata loadStart lostPointerCapture mouseDown mouseMove mouseOut mouseOver mouseUp paste pause play playing pointerCancel pointerDown pointerMove pointerOut pointerOver pointerUp progress rateChange reset resize seeked seeking stalled submit suspend timeUpdate touchCancel touchEnd touchStart volumeChange scroll toggle touchMove waiting wheel`.split(` `);Kr.push(`scrollEnd`);function V(e,t){Gr.set(e,t),xt(t,[e])}var qr=typeof reportError==`function`?reportError:function(e){if(typeof window==`object`&&typeof window.ErrorEvent==`function`){var t=new window.ErrorEvent(`error`,{bubbles:!0,cancelable:!0,message:typeof e==`object`&&e&&typeof e.message==`string`?String(e.message):String(e),error:e});if(!window.dispatchEvent(t))return}else if(typeof process==`object`&&typeof process.emit==`function`){process.emit(`uncaughtException`,e);return}console.error(e)},Jr=[],Yr=0,Xr=0;function Zr(){for(var e=Yr,t=Xr=Yr=0;t<e;){var n=Jr[t];Jr[t++]=null;var r=Jr[t];Jr[t++]=null;var i=Jr[t];Jr[t++]=null;var a=Jr[t];if(Jr[t++]=null,r!==null&&i!==null){var o=r.pending;o===null?i.next=i:(i.next=o.next,o.next=i),r.pending=i}a!==0&&ti(n,i,a)}}function Qr(e,t,n,r){Jr[Yr++]=e,Jr[Yr++]=t,Jr[Yr++]=n,Jr[Yr++]=r,Xr|=r,e.lanes|=r,e=e.alternate,e!==null&&(e.lanes|=r)}function $r(e,t,n,r){return Qr(e,t,n,r),ni(e)}function ei(e,t){return Qr(e,null,null,t),ni(e)}function ti(e,t,n){e.lanes|=n;var r=e.alternate;r!==null&&(r.lanes|=n);for(var i=!1,a=e.return;a!==null;)a.childLanes|=n,r=a.alternate,r!==null&&(r.childLanes|=n),a.tag===22&&(e=a.stateNode,e===null||e._visibility&1||(i=!0)),e=a,a=a.return;return e.tag===3?(a=e.stateNode,i&&t!==null&&(i=31-Ie(n),e=a.hiddenUpdates,r=e[i],r===null?e[i]=[t]:r.push(t),t.lane=n|536870912),a):null}function ni(e){if(50<du)throw du=0,fu=null,Error(a(185));for(var t=e.return;t!==null;)e=t,t=e.return;return e.tag===3?e.stateNode:null}var ri={};function ii(e,t,n,r){this.tag=e,this.key=n,this.sibling=this.child=this.return=this.stateNode=this.type=this.elementType=null,this.index=0,this.refCleanup=this.ref=null,this.pendingProps=t,this.dependencies=this.memoizedState=this.updateQueue=this.memoizedProps=null,this.mode=r,this.subtreeFlags=this.flags=0,this.deletions=null,this.childLanes=this.lanes=0,this.alternate=null}function ai(e,t,n,r){return new ii(e,t,n,r)}function oi(e){return e=e.prototype,!(!e||!e.isReactComponent)}function si(e,t){var n=e.alternate;return n===null?(n=ai(e.tag,t,e.key,e.mode),n.elementType=e.elementType,n.type=e.type,n.stateNode=e.stateNode,n.alternate=e,e.alternate=n):(n.pendingProps=t,n.type=e.type,n.flags=0,n.subtreeFlags=0,n.deletions=null),n.flags=e.flags&65011712,n.childLanes=e.childLanes,n.lanes=e.lanes,n.child=e.child,n.memoizedProps=e.memoizedProps,n.memoizedState=e.memoizedState,n.updateQueue=e.updateQueue,t=e.dependencies,n.dependencies=t===null?null:{lanes:t.lanes,firstContext:t.firstContext},n.sibling=e.sibling,n.index=e.index,n.ref=e.ref,n.refCleanup=e.refCleanup,n}function ci(e,t){e.flags&=65011714;var n=e.alternate;return n===null?(e.childLanes=0,e.lanes=t,e.child=null,e.subtreeFlags=0,e.memoizedProps=null,e.memoizedState=null,e.updateQueue=null,e.dependencies=null,e.stateNode=null):(e.childLanes=n.childLanes,e.lanes=n.lanes,e.child=n.child,e.subtreeFlags=0,e.deletions=null,e.memoizedProps=n.memoizedProps,e.memoizedState=n.memoizedState,e.updateQueue=n.updateQueue,e.type=n.type,t=n.dependencies,e.dependencies=t===null?null:{lanes:t.lanes,firstContext:t.firstContext}),e}function li(e,t,n,r,i,o){var s=0;if(r=e,typeof e==`function`)oi(e)&&(s=1);else if(typeof e==`string`)s=Uf(e,n,oe.current)?26:e===`html`||e===`head`||e===`body`?27:5;else a:switch(e){case D:return e=ai(31,n,t,i),e.elementType=D,e.lanes=o,e;case v:return ui(n.children,i,o,t);case y:s=8,i|=24;break;case b:return e=ai(12,n,t,i|2),e.elementType=b,e.lanes=o,e;case w:return e=ai(13,n,t,i),e.elementType=w,e.lanes=o,e;case ee:return e=ai(19,n,t,i),e.elementType=ee,e.lanes=o,e;default:if(typeof e==`object`&&e)switch(e.$$typeof){case S:s=10;break a;case x:s=9;break a;case C:s=11;break a;case T:s=14;break a;case E:s=16,r=null;break a}s=29,n=Error(a(130,e===null?`null`:typeof e,``)),r=null}return t=ai(s,n,t,i),t.elementType=e,t.type=r,t.lanes=o,t}function ui(e,t,n,r){return e=ai(7,e,r,t),e.lanes=n,e}function di(e,t,n){return e=ai(6,e,null,t),e.lanes=n,e}function fi(e){var t=ai(18,null,null,0);return t.stateNode=e,t}function pi(e,t,n){return t=ai(4,e.children===null?[]:e.children,e.key,t),t.lanes=n,t.stateNode={containerInfo:e.containerInfo,pendingChildren:null,implementation:e.implementation},t}var mi=new WeakMap;function hi(e,t){if(typeof e==`object`&&e){var n=mi.get(e);return n===void 0?(t={value:e,source:t,stack:ve(t)},mi.set(e,t),t):n}return{value:e,source:t,stack:ve(t)}}var gi=[],_i=0,vi=null,yi=0,bi=[],xi=0,Si=null,Ci=1,wi=``;function Ti(e,t){gi[_i++]=yi,gi[_i++]=vi,vi=e,yi=t}function Ei(e,t,n){bi[xi++]=Ci,bi[xi++]=wi,bi[xi++]=Si,Si=e;var r=Ci;e=wi;var i=32-Ie(r)-1;r&=~(1<<i),n+=1;var a=32-Ie(t)+i;if(30<a){var o=i-i%5;a=(r&(1<<o)-1).toString(32),r>>=o,i-=o,Ci=1<<32-Ie(t)+i|n<<i|r,wi=a+e}else Ci=1<<a|n<<i|r,wi=e}function Di(e){e.return!==null&&(Ti(e,1),Ei(e,1,0))}function Oi(e){for(;e===vi;)vi=gi[--_i],gi[_i]=null,yi=gi[--_i],gi[_i]=null;for(;e===Si;)Si=bi[--xi],bi[xi]=null,wi=bi[--xi],bi[xi]=null,Ci=bi[--xi],bi[xi]=null}function ki(e,t){bi[xi++]=Ci,bi[xi++]=wi,bi[xi++]=Si,Ci=t.id,wi=t.overflow,Si=e}var Ai=null,ji=null,H=!1,Mi=null,Ni=!1,Pi=Error(a(519));function Fi(e){throw Vi(hi(Error(a(418,1<arguments.length&&arguments[1]!==void 0&&arguments[1]?`text`:`HTML`,``)),e)),Pi}function Ii(e){var t=e.stateNode,n=e.type,r=e.memoizedProps;switch(t[at]=e,t[ot]=r,n){case`dialog`:$(`cancel`,t),$(`close`,t);break;case`iframe`:case`object`:case`embed`:$(`load`,t);break;case`video`:case`audio`:for(n=0;n<gd.length;n++)$(gd[n],t);break;case`source`:$(`error`,t);break;case`img`:case`image`:case`link`:$(`error`,t),$(`load`,t);break;case`details`:$(`toggle`,t);break;case`input`:$(`invalid`,t),zt(t,r.value,r.defaultValue,r.checked,r.defaultChecked,r.type,r.name,!0);break;case`select`:$(`invalid`,t);break;case`textarea`:$(`invalid`,t),Ut(t,r.value,r.defaultValue,r.children)}n=r.children,typeof n!=`string`&&typeof n!=`number`&&typeof n!=`bigint`||t.textContent===``+n||!0===r.suppressHydrationWarning||jd(t.textContent,n)?(r.popover!=null&&($(`beforetoggle`,t),$(`toggle`,t)),r.onScroll!=null&&$(`scroll`,t),r.onScrollEnd!=null&&$(`scrollend`,t),r.onClick!=null&&(t.onclick=Qt),t=!0):t=!1,t||Fi(e,!0)}function Li(e){for(Ai=e.return;Ai;)switch(Ai.tag){case 5:case 31:case 13:Ni=!1;return;case 27:case 3:Ni=!0;return;default:Ai=Ai.return}}function Ri(e){if(e!==Ai)return!1;if(!H)return Li(e),H=!0,!1;var t=e.tag,n;if((n=t!==3&&t!==27)&&((n=t===5)&&(n=e.type,n=!(n!==`form`&&n!==`button`)||Ud(e.type,e.memoizedProps)),n=!n),n&&ji&&Fi(e),Li(e),t===13){if(e=e.memoizedState,e=e===null?null:e.dehydrated,!e)throw Error(a(317));ji=uf(e)}else if(t===31){if(e=e.memoizedState,e=e===null?null:e.dehydrated,!e)throw Error(a(317));ji=uf(e)}else t===27?(t=ji,Zd(e.type)?(e=lf,lf=null,ji=e):ji=t):ji=Ai?cf(e.stateNode.nextSibling):null;return!0}function zi(){ji=Ai=null,H=!1}function Bi(){var e=Mi;return e!==null&&(Zl===null?Zl=e:Zl.push.apply(Zl,e),Mi=null),e}function Vi(e){Mi===null?Mi=[e]:Mi.push(e)}var Hi=ae(null),Ui=null,Wi=null;function Gi(e,t,n){L(Hi,t._currentValue),t._currentValue=n}function Ki(e){e._currentValue=Hi.current,I(Hi)}function qi(e,t,n){for(;e!==null;){var r=e.alternate;if((e.childLanes&t)===t?r!==null&&(r.childLanes&t)!==t&&(r.childLanes|=t):(e.childLanes|=t,r!==null&&(r.childLanes|=t)),e===n)break;e=e.return}}function Ji(e,t,n,r){var i=e.child;for(i!==null&&(i.return=e);i!==null;){var o=i.dependencies;if(o!==null){var s=i.child;o=o.firstContext;a:for(;o!==null;){var c=o;o=i;for(var l=0;l<t.length;l++)if(c.context===t[l]){o.lanes|=n,c=o.alternate,c!==null&&(c.lanes|=n),qi(o.return,n,e),r||(s=null);break a}o=c.next}}else if(i.tag===18){if(s=i.return,s===null)throw Error(a(341));s.lanes|=n,o=s.alternate,o!==null&&(o.lanes|=n),qi(s,n,e),s=null}else s=i.child;if(s!==null)s.return=i;else for(s=i;s!==null;){if(s===e){s=null;break}if(i=s.sibling,i!==null){i.return=s.return,s=i;break}s=s.return}i=s}}function Yi(e,t,n,r){e=null;for(var i=t,o=!1;i!==null;){if(!o){if(i.flags&524288)o=!0;else if(i.flags&262144)break}if(i.tag===10){var s=i.alternate;if(s===null)throw Error(a(387));if(s=s.memoizedProps,s!==null){var c=i.type;xr(i.pendingProps.value,s.value)||(e===null?e=[c]:e.push(c))}}else if(i===le.current){if(s=i.alternate,s===null)throw Error(a(387));s.memoizedState.memoizedState!==i.memoizedState.memoizedState&&(e===null?e=[Qf]:e.push(Qf))}i=i.return}e!==null&&Ji(t,e,n,r),t.flags|=262144}function Xi(e){for(e=e.firstContext;e!==null;){if(!xr(e.context._currentValue,e.memoizedValue))return!0;e=e.next}return!1}function Zi(e){Ui=e,Wi=null,e=e.dependencies,e!==null&&(e.firstContext=null)}function Qi(e){return ea(Ui,e)}function $i(e,t){return Ui===null&&Zi(e),ea(e,t)}function ea(e,t){var n=t._currentValue;if(t={context:t,memoizedValue:n,next:null},Wi===null){if(e===null)throw Error(a(308));Wi=t,e.dependencies={lanes:0,firstContext:t},e.flags|=524288}else Wi=Wi.next=t;return n}var ta=typeof AbortController<`u`?AbortController:function(){var e=[],t=this.signal={aborted:!1,addEventListener:function(t,n){e.push(n)}};this.abort=function(){t.aborted=!0,e.forEach(function(e){return e()})}},na=t.unstable_scheduleCallback,ra=t.unstable_NormalPriority,ia={$$typeof:S,Consumer:null,Provider:null,_currentValue:null,_currentValue2:null,_threadCount:0};function aa(){return{controller:new ta,data:new Map,refCount:0}}function oa(e){e.refCount--,e.refCount===0&&na(ra,function(){e.controller.abort()})}var sa=null,ca=0,la=0,ua=null;function da(e,t){if(sa===null){var n=sa=[];ca=0,la=ud(),ua={status:`pending`,value:void 0,then:function(e){n.push(e)}}}return ca++,t.then(fa,fa),t}function fa(){if(--ca===0&&sa!==null){ua!==null&&(ua.status=`fulfilled`);var e=sa;sa=null,la=0,ua=null;for(var t=0;t<e.length;t++)(0,e[t])()}}function pa(e,t){var n=[],r={status:`pending`,value:null,reason:null,then:function(e){n.push(e)}};return e.then(function(){r.status=`fulfilled`,r.value=t;for(var e=0;e<n.length;e++)(0,n[e])(t)},function(e){for(r.status=`rejected`,r.reason=e,e=0;e<n.length;e++)(0,n[e])(void 0)}),r}var ma=M.S;M.S=function(e,t){eu=we(),typeof t==`object`&&t&&typeof t.then==`function`&&da(e,t),ma!==null&&ma(e,t)};var ha=ae(null);function ga(){var e=ha.current;return e===null?Ll.pooledCache:e}function _a(e,t){t===null?L(ha,ha.current):L(ha,t.pool)}function va(){var e=ga();return e===null?null:{parent:ia._currentValue,pool:e}}var ya=Error(a(460)),ba=Error(a(474)),xa=Error(a(542)),Sa={then:function(){}};function Ca(e){return e=e.status,e===`fulfilled`||e===`rejected`}function wa(e,t,n){switch(n=e[n],n===void 0?e.push(t):n!==t&&(t.then(Qt,Qt),t=n),t.status){case`fulfilled`:return t.value;case`rejected`:throw e=t.reason,Oa(e),e;default:if(typeof t.status==`string`)t.then(Qt,Qt);else{if(e=Ll,e!==null&&100<e.shellSuspendCounter)throw Error(a(482));e=t,e.status=`pending`,e.then(function(e){if(t.status===`pending`){var n=t;n.status=`fulfilled`,n.value=e}},function(e){if(t.status===`pending`){var n=t;n.status=`rejected`,n.reason=e}})}switch(t.status){case`fulfilled`:return t.value;case`rejected`:throw e=t.reason,Oa(e),e}throw Ea=t,ya}}function Ta(e){try{var t=e._init;return t(e._payload)}catch(e){throw typeof e==`object`&&e&&typeof e.then==`function`?(Ea=e,ya):e}}var Ea=null;function Da(){if(Ea===null)throw Error(a(459));var e=Ea;return Ea=null,e}function Oa(e){if(e===ya||e===xa)throw Error(a(483))}var ka=null,Aa=0;function ja(e){var t=Aa;return Aa+=1,ka===null&&(ka=[]),wa(ka,e,t)}function Ma(e,t){t=t.props.ref,e.ref=t===void 0?null:t}function Na(e,t){throw t.$$typeof===h?Error(a(525)):(e=Object.prototype.toString.call(t),Error(a(31,e===`[object Object]`?`object with keys {`+Object.keys(t).join(`, `)+`}`:e)))}function Pa(e){function t(t,n){if(e){var r=t.deletions;r===null?(t.deletions=[n],t.flags|=16):r.push(n)}}function n(n,r){if(!e)return null;for(;r!==null;)t(n,r),r=r.sibling;return null}function r(e){for(var t=new Map;e!==null;)e.key===null?t.set(e.index,e):t.set(e.key,e),e=e.sibling;return t}function i(e,t){return e=si(e,t),e.index=0,e.sibling=null,e}function o(t,n,r){return t.index=r,e?(r=t.alternate,r===null?(t.flags|=67108866,n):(r=r.index,r<n?(t.flags|=67108866,n):r)):(t.flags|=1048576,n)}function s(t){return e&&t.alternate===null&&(t.flags|=67108866),t}function c(e,t,n,r){return t===null||t.tag!==6?(t=di(n,e.mode,r),t.return=e,t):(t=i(t,n),t.return=e,t)}function l(e,t,n,r){var a=n.type;return a===v?d(e,t,n.props.children,r,n.key):t!==null&&(t.elementType===a||typeof a==`object`&&a&&a.$$typeof===E&&Ta(a)===t.type)?(t=i(t,n.props),Ma(t,n),t.return=e,t):(t=li(n.type,n.key,n.props,null,e.mode,r),Ma(t,n),t.return=e,t)}function u(e,t,n,r){return t===null||t.tag!==4||t.stateNode.containerInfo!==n.containerInfo||t.stateNode.implementation!==n.implementation?(t=pi(n,e.mode,r),t.return=e,t):(t=i(t,n.children||[]),t.return=e,t)}function d(e,t,n,r,a){return t===null||t.tag!==7?(t=ui(n,e.mode,r,a),t.return=e,t):(t=i(t,n),t.return=e,t)}function f(e,t,n){if(typeof t==`string`&&t!==``||typeof t==`number`||typeof t==`bigint`)return t=di(``+t,e.mode,n),t.return=e,t;if(typeof t==`object`&&t){switch(t.$$typeof){case g:return n=li(t.type,t.key,t.props,null,e.mode,n),Ma(n,t),n.return=e,n;case _:return t=pi(t,e.mode,n),t.return=e,t;case E:return t=Ta(t),f(e,t,n)}if(re(t)||A(t))return t=ui(t,e.mode,n,null),t.return=e,t;if(typeof t.then==`function`)return f(e,ja(t),n);if(t.$$typeof===S)return f(e,$i(e,t),n);Na(e,t)}return null}function p(e,t,n,r){var i=t===null?null:t.key;if(typeof n==`string`&&n!==``||typeof n==`number`||typeof n==`bigint`)return i===null?c(e,t,``+n,r):null;if(typeof n==`object`&&n){switch(n.$$typeof){case g:return n.key===i?l(e,t,n,r):null;case _:return n.key===i?u(e,t,n,r):null;case E:return n=Ta(n),p(e,t,n,r)}if(re(n)||A(n))return i===null?d(e,t,n,r,null):null;if(typeof n.then==`function`)return p(e,t,ja(n),r);if(n.$$typeof===S)return p(e,t,$i(e,n),r);Na(e,n)}return null}function m(e,t,n,r,i){if(typeof r==`string`&&r!==``||typeof r==`number`||typeof r==`bigint`)return e=e.get(n)||null,c(t,e,``+r,i);if(typeof r==`object`&&r){switch(r.$$typeof){case g:return e=e.get(r.key===null?n:r.key)||null,l(t,e,r,i);case _:return e=e.get(r.key===null?n:r.key)||null,u(t,e,r,i);case E:return r=Ta(r),m(e,t,n,r,i)}if(re(r)||A(r))return e=e.get(n)||null,d(t,e,r,i,null);if(typeof r.then==`function`)return m(e,t,n,ja(r),i);if(r.$$typeof===S)return m(e,t,n,$i(t,r),i);Na(t,r)}return null}function h(i,a,s,c){for(var l=null,u=null,d=a,h=a=0,g=null;d!==null&&h<s.length;h++){d.index>h?(g=d,d=null):g=d.sibling;var _=p(i,d,s[h],c);if(_===null){d===null&&(d=g);break}e&&d&&_.alternate===null&&t(i,d),a=o(_,a,h),u===null?l=_:u.sibling=_,u=_,d=g}if(h===s.length)return n(i,d),H&&Ti(i,h),l;if(d===null){for(;h<s.length;h++)d=f(i,s[h],c),d!==null&&(a=o(d,a,h),u===null?l=d:u.sibling=d,u=d);return H&&Ti(i,h),l}for(d=r(d);h<s.length;h++)g=m(d,i,h,s[h],c),g!==null&&(e&&g.alternate!==null&&d.delete(g.key===null?h:g.key),a=o(g,a,h),u===null?l=g:u.sibling=g,u=g);return e&&d.forEach(function(e){return t(i,e)}),H&&Ti(i,h),l}function y(i,s,c,l){if(c==null)throw Error(a(151));for(var u=null,d=null,h=s,g=s=0,_=null,v=c.next();h!==null&&!v.done;g++,v=c.next()){h.index>g?(_=h,h=null):_=h.sibling;var y=p(i,h,v.value,l);if(y===null){h===null&&(h=_);break}e&&h&&y.alternate===null&&t(i,h),s=o(y,s,g),d===null?u=y:d.sibling=y,d=y,h=_}if(v.done)return n(i,h),H&&Ti(i,g),u;if(h===null){for(;!v.done;g++,v=c.next())v=f(i,v.value,l),v!==null&&(s=o(v,s,g),d===null?u=v:d.sibling=v,d=v);return H&&Ti(i,g),u}for(h=r(h);!v.done;g++,v=c.next())v=m(h,i,g,v.value,l),v!==null&&(e&&v.alternate!==null&&h.delete(v.key===null?g:v.key),s=o(v,s,g),d===null?u=v:d.sibling=v,d=v);return e&&h.forEach(function(e){return t(i,e)}),H&&Ti(i,g),u}function b(e,r,o,c){if(typeof o==`object`&&o&&o.type===v&&o.key===null&&(o=o.props.children),typeof o==`object`&&o){switch(o.$$typeof){case g:a:{for(var l=o.key;r!==null;){if(r.key===l){if(l=o.type,l===v){if(r.tag===7){n(e,r.sibling),c=i(r,o.props.children),c.return=e,e=c;break a}}else if(r.elementType===l||typeof l==`object`&&l&&l.$$typeof===E&&Ta(l)===r.type){n(e,r.sibling),c=i(r,o.props),Ma(c,o),c.return=e,e=c;break a}n(e,r);break}else t(e,r);r=r.sibling}o.type===v?(c=ui(o.props.children,e.mode,c,o.key),c.return=e,e=c):(c=li(o.type,o.key,o.props,null,e.mode,c),Ma(c,o),c.return=e,e=c)}return s(e);case _:a:{for(l=o.key;r!==null;){if(r.key===l)if(r.tag===4&&r.stateNode.containerInfo===o.containerInfo&&r.stateNode.implementation===o.implementation){n(e,r.sibling),c=i(r,o.children||[]),c.return=e,e=c;break a}else{n(e,r);break}else t(e,r);r=r.sibling}c=pi(o,e.mode,c),c.return=e,e=c}return s(e);case E:return o=Ta(o),b(e,r,o,c)}if(re(o))return h(e,r,o,c);if(A(o)){if(l=A(o),typeof l!=`function`)throw Error(a(150));return o=l.call(o),y(e,r,o,c)}if(typeof o.then==`function`)return b(e,r,ja(o),c);if(o.$$typeof===S)return b(e,r,$i(e,o),c);Na(e,o)}return typeof o==`string`&&o!==``||typeof o==`number`||typeof o==`bigint`?(o=``+o,r!==null&&r.tag===6?(n(e,r.sibling),c=i(r,o),c.return=e,e=c):(n(e,r),c=di(o,e.mode,c),c.return=e,e=c),s(e)):n(e,r)}return function(e,t,n,r){try{Aa=0;var i=b(e,t,n,r);return ka=null,i}catch(t){if(t===ya||t===xa)throw t;var a=ai(29,t,null,e.mode);return a.lanes=r,a.return=e,a}}}var Fa=Pa(!0),Ia=Pa(!1),La=!1;function Ra(e){e.updateQueue={baseState:e.memoizedState,firstBaseUpdate:null,lastBaseUpdate:null,shared:{pending:null,lanes:0,hiddenCallbacks:null},callbacks:null}}function za(e,t){e=e.updateQueue,t.updateQueue===e&&(t.updateQueue={baseState:e.baseState,firstBaseUpdate:e.firstBaseUpdate,lastBaseUpdate:e.lastBaseUpdate,shared:e.shared,callbacks:null})}function Ba(e){return{lane:e,tag:0,payload:null,callback:null,next:null}}function Va(e,t,n){var r=e.updateQueue;if(r===null)return null;if(r=r.shared,Il&2){var i=r.pending;return i===null?t.next=t:(t.next=i.next,i.next=t),r.pending=t,t=ni(e),ti(e,null,n),t}return Qr(e,r,t,n),ni(e)}function Ha(e,t,n){if(t=t.updateQueue,t!==null&&(t=t.shared,n&4194048)){var r=t.lanes;r&=e.pendingLanes,n|=r,t.lanes=n,Qe(e,n)}}function Ua(e,t){var n=e.updateQueue,r=e.alternate;if(r!==null&&(r=r.updateQueue,n===r)){var i=null,a=null;if(n=n.firstBaseUpdate,n!==null){do{var o={lane:n.lane,tag:n.tag,payload:n.payload,callback:null,next:null};a===null?i=a=o:a=a.next=o,n=n.next}while(n!==null);a===null?i=a=t:a=a.next=t}else i=a=t;n={baseState:r.baseState,firstBaseUpdate:i,lastBaseUpdate:a,shared:r.shared,callbacks:r.callbacks},e.updateQueue=n;return}e=n.lastBaseUpdate,e===null?n.firstBaseUpdate=t:e.next=t,n.lastBaseUpdate=t}var Wa=!1;function Ga(){if(Wa){var e=ua;if(e!==null)throw e}}function Ka(e,t,n,r){Wa=!1;var i=e.updateQueue;La=!1;var a=i.firstBaseUpdate,o=i.lastBaseUpdate,s=i.shared.pending;if(s!==null){i.shared.pending=null;var c=s,l=c.next;c.next=null,o===null?a=l:o.next=l,o=c;var u=e.alternate;u!==null&&(u=u.updateQueue,s=u.lastBaseUpdate,s!==o&&(s===null?u.firstBaseUpdate=l:s.next=l,u.lastBaseUpdate=c))}if(a!==null){var d=i.baseState;o=0,u=l=c=null,s=a;do{var f=s.lane&-536870913,p=f!==s.lane;if(p?(X&f)===f:(r&f)===f){f!==0&&f===la&&(Wa=!0),u!==null&&(u=u.next={lane:0,tag:s.tag,payload:s.payload,callback:null,next:null});a:{var h=e,g=s;f=t;var _=n;switch(g.tag){case 1:if(h=g.payload,typeof h==`function`){d=h.call(_,d,f);break a}d=h;break a;case 3:h.flags=h.flags&-65537|128;case 0:if(h=g.payload,f=typeof h==`function`?h.call(_,d,f):h,f==null)break a;d=m({},d,f);break a;case 2:La=!0}}f=s.callback,f!==null&&(e.flags|=64,p&&(e.flags|=8192),p=i.callbacks,p===null?i.callbacks=[f]:p.push(f))}else p={lane:f,tag:s.tag,payload:s.payload,callback:s.callback,next:null},u===null?(l=u=p,c=d):u=u.next=p,o|=f;if(s=s.next,s===null){if(s=i.shared.pending,s===null)break;p=s,s=p.next,p.next=null,i.lastBaseUpdate=p,i.shared.pending=null}}while(1);u===null&&(c=d),i.baseState=c,i.firstBaseUpdate=l,i.lastBaseUpdate=u,a===null&&(i.shared.lanes=0),Gl|=o,e.lanes=o,e.memoizedState=d}}function qa(e,t){if(typeof e!=`function`)throw Error(a(191,e));e.call(t)}function Ja(e,t){var n=e.callbacks;if(n!==null)for(e.callbacks=null,e=0;e<n.length;e++)qa(n[e],t)}var Ya=ae(null),Xa=ae(0);function Za(e,t){e=Ul,L(Xa,e),L(Ya,t),Ul=e|t.baseLanes}function Qa(){L(Xa,Ul),L(Ya,Ya.current)}function $a(){Ul=Xa.current,I(Ya),I(Xa)}var eo=ae(null),to=null;function no(e){var t=e.alternate;L(so,so.current&1),L(eo,e),to===null&&(t===null||Ya.current!==null||t.memoizedState!==null)&&(to=e)}function ro(e){L(so,so.current),L(eo,e),to===null&&(to=e)}function io(e){e.tag===22?(L(so,so.current),L(eo,e),to===null&&(to=e)):ao(e)}function ao(){L(so,so.current),L(eo,eo.current)}function oo(e){I(eo),to===e&&(to=null),I(so)}var so=ae(0);function co(e){for(var t=e;t!==null;){if(t.tag===13){var n=t.memoizedState;if(n!==null&&(n=n.dehydrated,n===null||af(n)||of(n)))return t}else if(t.tag===19&&(t.memoizedProps.revealOrder===`forwards`||t.memoizedProps.revealOrder===`backwards`||t.memoizedProps.revealOrder===`unstable_legacy-backwards`||t.memoizedProps.revealOrder===`together`)){if(t.flags&128)return t}else if(t.child!==null){t.child.return=t,t=t.child;continue}if(t===e)break;for(;t.sibling===null;){if(t.return===null||t.return===e)return null;t=t.return}t.sibling.return=t.return,t=t.sibling}return null}var lo=0,U=null,uo=null,fo=null,po=!1,mo=!1,ho=!1,go=0,_o=0,vo=null,yo=0;function bo(){throw Error(a(321))}function xo(e,t){if(t===null)return!1;for(var n=0;n<t.length&&n<e.length;n++)if(!xr(e[n],t[n]))return!1;return!0}function So(e,t,n,r,i,a){return lo=a,U=t,t.memoizedState=null,t.updateQueue=null,t.lanes=0,M.H=e===null||e.memoizedState===null?Fs:Is,ho=!1,a=n(r,i),ho=!1,mo&&(a=wo(t,n,r,i)),Co(e),a}function Co(e){M.H=Ps;var t=uo!==null&&uo.next!==null;if(lo=0,fo=uo=U=null,po=!1,_o=0,vo=null,t)throw Error(a(300));e===null||Qs||(e=e.dependencies,e!==null&&Xi(e)&&(Qs=!0))}function wo(e,t,n,r){U=e;var i=0;do{if(mo&&(vo=null),_o=0,mo=!1,25<=i)throw Error(a(301));if(i+=1,fo=uo=null,e.updateQueue!=null){var o=e.updateQueue;o.lastEffect=null,o.events=null,o.stores=null,o.memoCache!=null&&(o.memoCache.index=0)}M.H=Ls,o=t(n,r)}while(mo);return o}function To(){var e=M.H,t=e.useState()[0];return t=typeof t.then==`function`?Mo(t):t,e=e.useState()[0],(uo===null?null:uo.memoizedState)!==e&&(U.flags|=1024),t}function Eo(){var e=go!==0;return go=0,e}function Do(e,t,n){t.updateQueue=e.updateQueue,t.flags&=-2053,e.lanes&=~n}function Oo(e){if(po){for(e=e.memoizedState;e!==null;){var t=e.queue;t!==null&&(t.pending=null),e=e.next}po=!1}lo=0,fo=uo=U=null,mo=!1,_o=go=0,vo=null}function ko(){var e={memoizedState:null,baseState:null,baseQueue:null,queue:null,next:null};return fo===null?U.memoizedState=fo=e:fo=fo.next=e,fo}function Ao(){if(uo===null){var e=U.alternate;e=e===null?null:e.memoizedState}else e=uo.next;var t=fo===null?U.memoizedState:fo.next;if(t!==null)fo=t,uo=e;else{if(e===null)throw U.alternate===null?Error(a(467)):Error(a(310));uo=e,e={memoizedState:uo.memoizedState,baseState:uo.baseState,baseQueue:uo.baseQueue,queue:uo.queue,next:null},fo===null?U.memoizedState=fo=e:fo=fo.next=e}return fo}function jo(){return{lastEffect:null,events:null,stores:null,memoCache:null}}function Mo(e){var t=_o;return _o+=1,vo===null&&(vo=[]),e=wa(vo,e,t),t=U,(fo===null?t.memoizedState:fo.next)===null&&(t=t.alternate,M.H=t===null||t.memoizedState===null?Fs:Is),e}function No(e){if(typeof e==`object`&&e){if(typeof e.then==`function`)return Mo(e);if(e.$$typeof===S)return Qi(e)}throw Error(a(438,String(e)))}function Po(e){var t=null,n=U.updateQueue;if(n!==null&&(t=n.memoCache),t==null){var r=U.alternate;r!==null&&(r=r.updateQueue,r!==null&&(r=r.memoCache,r!=null&&(t={data:r.data.map(function(e){return e.slice()}),index:0})))}if(t??={data:[],index:0},n===null&&(n=jo(),U.updateQueue=n),n.memoCache=t,n=t.data[t.index],n===void 0)for(n=t.data[t.index]=Array(e),r=0;r<e;r++)n[r]=O;return t.index++,n}function Fo(e,t){return typeof t==`function`?t(e):t}function Io(e){return Lo(Ao(),uo,e)}function Lo(e,t,n){var r=e.queue;if(r===null)throw Error(a(311));r.lastRenderedReducer=n;var i=e.baseQueue,o=r.pending;if(o!==null){if(i!==null){var s=i.next;i.next=o.next,o.next=s}t.baseQueue=i=o,r.pending=null}if(o=e.baseState,i===null)e.memoizedState=o;else{t=i.next;var c=s=null,l=null,u=t,d=!1;do{var f=u.lane&-536870913;if(f===u.lane?(lo&f)===f:(X&f)===f){var p=u.revertLane;if(p===0)l!==null&&(l=l.next={lane:0,revertLane:0,gesture:null,action:u.action,hasEagerState:u.hasEagerState,eagerState:u.eagerState,next:null}),f===la&&(d=!0);else if((lo&p)===p){u=u.next,p===la&&(d=!0);continue}else f={lane:0,revertLane:u.revertLane,gesture:null,action:u.action,hasEagerState:u.hasEagerState,eagerState:u.eagerState,next:null},l===null?(c=l=f,s=o):l=l.next=f,U.lanes|=p,Gl|=p;f=u.action,ho&&n(o,f),o=u.hasEagerState?u.eagerState:n(o,f)}else p={lane:f,revertLane:u.revertLane,gesture:u.gesture,action:u.action,hasEagerState:u.hasEagerState,eagerState:u.eagerState,next:null},l===null?(c=l=p,s=o):l=l.next=p,U.lanes|=f,Gl|=f;u=u.next}while(u!==null&&u!==t);if(l===null?s=o:l.next=c,!xr(o,e.memoizedState)&&(Qs=!0,d&&(n=ua,n!==null)))throw n;e.memoizedState=o,e.baseState=s,e.baseQueue=l,r.lastRenderedState=o}return i===null&&(r.lanes=0),[e.memoizedState,r.dispatch]}function Ro(e){var t=Ao(),n=t.queue;if(n===null)throw Error(a(311));n.lastRenderedReducer=e;var r=n.dispatch,i=n.pending,o=t.memoizedState;if(i!==null){n.pending=null;var s=i=i.next;do o=e(o,s.action),s=s.next;while(s!==i);xr(o,t.memoizedState)||(Qs=!0),t.memoizedState=o,t.baseQueue===null&&(t.baseState=o),n.lastRenderedState=o}return[o,r]}function zo(e,t,n){var r=U,i=Ao(),o=H;if(o){if(n===void 0)throw Error(a(407));n=n()}else n=t();var s=!xr((uo||i).memoizedState,n);if(s&&(i.memoizedState=n,Qs=!0),i=i.queue,ls(Ho.bind(null,r,i,e),[e]),i.getSnapshot!==t||s||fo!==null&&fo.memoizedState.tag&1){if(r.flags|=2048,as(9,{destroy:void 0},Vo.bind(null,r,i,n,t),null),Ll===null)throw Error(a(349));o||lo&127||Bo(r,t,n)}return n}function Bo(e,t,n){e.flags|=16384,e={getSnapshot:t,value:n},t=U.updateQueue,t===null?(t=jo(),U.updateQueue=t,t.stores=[e]):(n=t.stores,n===null?t.stores=[e]:n.push(e))}function Vo(e,t,n,r){t.value=n,t.getSnapshot=r,Uo(t)&&Wo(e)}function Ho(e,t,n){return n(function(){Uo(t)&&Wo(e)})}function Uo(e){var t=e.getSnapshot;e=e.value;try{var n=t();return!xr(e,n)}catch{return!0}}function Wo(e){var t=ei(e,2);t!==null&&hu(t,e,2)}function Go(e){var t=ko();if(typeof e==`function`){var n=e;if(e=n(),ho){Fe(!0);try{n()}finally{Fe(!1)}}}return t.memoizedState=t.baseState=e,t.queue={pending:null,lanes:0,dispatch:null,lastRenderedReducer:Fo,lastRenderedState:e},t}function Ko(e,t,n,r){return e.baseState=n,Lo(e,uo,typeof r==`function`?r:Fo)}function qo(e,t,n,r,i){if(js(e))throw Error(a(485));if(e=t.action,e!==null){var o={payload:i,action:e,next:null,isTransition:!0,status:`pending`,value:null,reason:null,listeners:[],then:function(e){o.listeners.push(e)}};M.T===null?o.isTransition=!1:n(!0),r(o),n=t.pending,n===null?(o.next=t.pending=o,Jo(t,o)):(o.next=n.next,t.pending=n.next=o)}}function Jo(e,t){var n=t.action,r=t.payload,i=e.state;if(t.isTransition){var a=M.T,o={};M.T=o;try{var s=n(i,r),c=M.S;c!==null&&c(o,s),Yo(e,t,s)}catch(n){Zo(e,t,n)}finally{a!==null&&o.types!==null&&(a.types=o.types),M.T=a}}else try{a=n(i,r),Yo(e,t,a)}catch(n){Zo(e,t,n)}}function Yo(e,t,n){typeof n==`object`&&n&&typeof n.then==`function`?n.then(function(n){Xo(e,t,n)},function(n){return Zo(e,t,n)}):Xo(e,t,n)}function Xo(e,t,n){t.status=`fulfilled`,t.value=n,Qo(t),e.state=n,t=e.pending,t!==null&&(n=t.next,n===t?e.pending=null:(n=n.next,t.next=n,Jo(e,n)))}function Zo(e,t,n){var r=e.pending;if(e.pending=null,r!==null){r=r.next;do t.status=`rejected`,t.reason=n,Qo(t),t=t.next;while(t!==r)}e.action=null}function Qo(e){e=e.listeners;for(var t=0;t<e.length;t++)(0,e[t])()}function $o(e,t){return t}function es(e,t){if(H){var n=Ll.formState;if(n!==null){a:{var r=U;if(H){if(ji){b:{for(var i=ji,a=Ni;i.nodeType!==8;){if(!a){i=null;break b}if(i=cf(i.nextSibling),i===null){i=null;break b}}a=i.data,i=a===`F!`||a===`F`?i:null}if(i){ji=cf(i.nextSibling),r=i.data===`F!`;break a}}Fi(r)}r=!1}r&&(t=n[0])}}return n=ko(),n.memoizedState=n.baseState=t,r={pending:null,lanes:0,dispatch:null,lastRenderedReducer:$o,lastRenderedState:t},n.queue=r,n=ks.bind(null,U,r),r.dispatch=n,r=Go(!1),a=q.bind(null,U,!1,r.queue),r=ko(),i={state:t,dispatch:null,action:e,pending:null},r.queue=i,n=qo.bind(null,U,i,a,n),i.dispatch=n,r.memoizedState=e,[t,n,!1]}function ts(e){return ns(Ao(),uo,e)}function ns(e,t,n){if(t=Lo(e,t,$o)[0],e=Io(Fo)[0],typeof t==`object`&&t&&typeof t.then==`function`)try{var r=Mo(t)}catch(e){throw e===ya?xa:e}else r=t;t=Ao();var i=t.queue,a=i.dispatch;return n!==t.memoizedState&&(U.flags|=2048,as(9,{destroy:void 0},rs.bind(null,i,n),null)),[r,a,e]}function rs(e,t){e.action=t}function is(e){var t=Ao(),n=uo;if(n!==null)return ns(t,n,e);Ao(),t=t.memoizedState,n=Ao();var r=n.queue.dispatch;return n.memoizedState=e,[t,r,!1]}function as(e,t,n,r){return e={tag:e,create:n,deps:r,inst:t,next:null},t=U.updateQueue,t===null&&(t=jo(),U.updateQueue=t),n=t.lastEffect,n===null?t.lastEffect=e.next=e:(r=n.next,n.next=e,e.next=r,t.lastEffect=e),e}function os(){return Ao().memoizedState}function W(e,t,n,r){var i=ko();U.flags|=e,i.memoizedState=as(1|t,{destroy:void 0},n,r===void 0?null:r)}function ss(e,t,n,r){var i=Ao();r=r===void 0?null:r;var a=i.memoizedState.inst;uo!==null&&r!==null&&xo(r,uo.memoizedState.deps)?i.memoizedState=as(t,a,n,r):(U.flags|=e,i.memoizedState=as(1|t,a,n,r))}function cs(e,t){W(8390656,8,e,t)}function ls(e,t){ss(2048,8,e,t)}function us(e){U.flags|=4;var t=U.updateQueue;if(t===null)t=jo(),U.updateQueue=t,t.events=[e];else{var n=t.events;n===null?t.events=[e]:n.push(e)}}function ds(e){var t=Ao().memoizedState;return us({ref:t,nextImpl:e}),function(){if(Il&2)throw Error(a(440));return t.impl.apply(void 0,arguments)}}function fs(e,t){return ss(4,2,e,t)}function ps(e,t){return ss(4,4,e,t)}function ms(e,t){if(typeof t==`function`){e=e();var n=t(e);return function(){typeof n==`function`?n():t(null)}}if(t!=null)return e=e(),t.current=e,function(){t.current=null}}function hs(e,t,n){n=n==null?null:n.concat([e]),ss(4,4,ms.bind(null,t,e),n)}function gs(){}function _s(e,t){var n=Ao();t=t===void 0?null:t;var r=n.memoizedState;return t!==null&&xo(t,r[1])?r[0]:(n.memoizedState=[e,t],e)}function G(e,t){var n=Ao();t=t===void 0?null:t;var r=n.memoizedState;if(t!==null&&xo(t,r[1]))return r[0];if(r=e(),ho){Fe(!0);try{e()}finally{Fe(!1)}}return n.memoizedState=[r,t],r}function vs(e,t,n){return n===void 0||lo&1073741824&&!(X&261930)?e.memoizedState=t:(e.memoizedState=n,e=mu(),U.lanes|=e,Gl|=e,n)}function ys(e,t,n,r){return xr(n,t)?n:Ya.current===null?!(lo&42)||lo&1073741824&&!(X&261930)?(Qs=!0,e.memoizedState=n):(e=mu(),U.lanes|=e,Gl|=e,t):(e=vs(e,n,r),xr(e,t)||(Qs=!0),e)}function bs(e,t,n,r,i){var a=N.p;N.p=a!==0&&8>a?a:8;var o=M.T,s={};M.T=s,q(e,!1,t,n);try{var c=i(),l=M.S;l!==null&&l(s,c),typeof c==`object`&&c&&typeof c.then==`function`?As(e,t,pa(c,r),pu(e)):As(e,t,r,pu(e))}catch(n){As(e,t,{then:function(){},status:`rejected`,reason:n},pu())}finally{N.p=a,o!==null&&s.types!==null&&(o.types=s.types),M.T=o}}function xs(){}function Ss(e,t,n,r){if(e.tag!==5)throw Error(a(476));var i=K(e).queue;bs(e,i,t,P,n===null?xs:function(){return Cs(e),n(r)})}function K(e){var t=e.memoizedState;if(t!==null)return t;t={memoizedState:P,baseState:P,baseQueue:null,queue:{pending:null,lanes:0,dispatch:null,lastRenderedReducer:Fo,lastRenderedState:P},next:null};var n={};return t.next={memoizedState:n,baseState:n,baseQueue:null,queue:{pending:null,lanes:0,dispatch:null,lastRenderedReducer:Fo,lastRenderedState:n},next:null},e.memoizedState=t,e=e.alternate,e!==null&&(e.memoizedState=t),t}function Cs(e){var t=K(e);t.next===null&&(t=e.alternate.memoizedState),As(e,t.next.queue,{},pu())}function ws(){return Qi(Qf)}function Ts(){return Ao().memoizedState}function Es(){return Ao().memoizedState}function Ds(e){for(var t=e.return;t!==null;){switch(t.tag){case 24:case 3:var n=pu();e=Ba(n);var r=Va(t,e,n);r!==null&&(hu(r,t,n),Ha(r,t,n)),t={cache:aa()},e.payload=t;return}t=t.return}}function Os(e,t,n){var r=pu();n={lane:r,revertLane:0,gesture:null,action:n,hasEagerState:!1,eagerState:null,next:null},js(e)?Ms(t,n):(n=$r(e,t,n,r),n!==null&&(hu(n,e,r),Ns(n,t,r)))}function ks(e,t,n){As(e,t,n,pu())}function As(e,t,n,r){var i={lane:r,revertLane:0,gesture:null,action:n,hasEagerState:!1,eagerState:null,next:null};if(js(e))Ms(t,i);else{var a=e.alternate;if(e.lanes===0&&(a===null||a.lanes===0)&&(a=t.lastRenderedReducer,a!==null))try{var o=t.lastRenderedState,s=a(o,n);if(i.hasEagerState=!0,i.eagerState=s,xr(s,o))return Qr(e,t,i,0),Ll===null&&Zr(),!1}catch{}if(n=$r(e,t,i,r),n!==null)return hu(n,e,r),Ns(n,t,r),!0}return!1}function q(e,t,n,r){if(r={lane:2,revertLane:ud(),gesture:null,action:r,hasEagerState:!1,eagerState:null,next:null},js(e)){if(t)throw Error(a(479))}else t=$r(e,n,r,2),t!==null&&hu(t,e,2)}function js(e){var t=e.alternate;return e===U||t!==null&&t===U}function Ms(e,t){mo=po=!0;var n=e.pending;n===null?t.next=t:(t.next=n.next,n.next=t),e.pending=t}function Ns(e,t,n){if(n&4194048){var r=t.lanes;r&=e.pendingLanes,n|=r,t.lanes=n,Qe(e,n)}}var Ps={readContext:Qi,use:No,useCallback:bo,useContext:bo,useEffect:bo,useImperativeHandle:bo,useLayoutEffect:bo,useInsertionEffect:bo,useMemo:bo,useReducer:bo,useRef:bo,useState:bo,useDebugValue:bo,useDeferredValue:bo,useTransition:bo,useSyncExternalStore:bo,useId:bo,useHostTransitionStatus:bo,useFormState:bo,useActionState:bo,useOptimistic:bo,useMemoCache:bo,useCacheRefresh:bo};Ps.useEffectEvent=bo;var Fs={readContext:Qi,use:No,useCallback:function(e,t){return ko().memoizedState=[e,t===void 0?null:t],e},useContext:Qi,useEffect:cs,useImperativeHandle:function(e,t,n){n=n==null?null:n.concat([e]),W(4194308,4,ms.bind(null,t,e),n)},useLayoutEffect:function(e,t){return W(4194308,4,e,t)},useInsertionEffect:function(e,t){W(4,2,e,t)},useMemo:function(e,t){var n=ko();t=t===void 0?null:t;var r=e();if(ho){Fe(!0);try{e()}finally{Fe(!1)}}return n.memoizedState=[r,t],r},useReducer:function(e,t,n){var r=ko();if(n!==void 0){var i=n(t);if(ho){Fe(!0);try{n(t)}finally{Fe(!1)}}}else i=t;return r.memoizedState=r.baseState=i,e={pending:null,lanes:0,dispatch:null,lastRenderedReducer:e,lastRenderedState:i},r.queue=e,e=e.dispatch=Os.bind(null,U,e),[r.memoizedState,e]},useRef:function(e){var t=ko();return e={current:e},t.memoizedState=e},useState:function(e){e=Go(e);var t=e.queue,n=ks.bind(null,U,t);return t.dispatch=n,[e.memoizedState,n]},useDebugValue:gs,useDeferredValue:function(e,t){return vs(ko(),e,t)},useTransition:function(){var e=Go(!1);return e=bs.bind(null,U,e.queue,!0,!1),ko().memoizedState=e,[!1,e]},useSyncExternalStore:function(e,t,n){var r=U,i=ko();if(H){if(n===void 0)throw Error(a(407));n=n()}else{if(n=t(),Ll===null)throw Error(a(349));X&127||Bo(r,t,n)}i.memoizedState=n;var o={value:n,getSnapshot:t};return i.queue=o,cs(Ho.bind(null,r,o,e),[e]),r.flags|=2048,as(9,{destroy:void 0},Vo.bind(null,r,o,n,t),null),n},useId:function(){var e=ko(),t=Ll.identifierPrefix;if(H){var n=wi,r=Ci;n=(r&~(1<<32-Ie(r)-1)).toString(32)+n,t=`_`+t+`R_`+n,n=go++,0<n&&(t+=`H`+n.toString(32)),t+=`_`}else n=yo++,t=`_`+t+`r_`+n.toString(32)+`_`;return e.memoizedState=t},useHostTransitionStatus:ws,useFormState:es,useActionState:es,useOptimistic:function(e){var t=ko();t.memoizedState=t.baseState=e;var n={pending:null,lanes:0,dispatch:null,lastRenderedReducer:null,lastRenderedState:null};return t.queue=n,t=q.bind(null,U,!0,n),n.dispatch=t,[e,t]},useMemoCache:Po,useCacheRefresh:function(){return ko().memoizedState=Ds.bind(null,U)},useEffectEvent:function(e){var t=ko(),n={impl:e};return t.memoizedState=n,function(){if(Il&2)throw Error(a(440));return n.impl.apply(void 0,arguments)}}},Is={readContext:Qi,use:No,useCallback:_s,useContext:Qi,useEffect:ls,useImperativeHandle:hs,useInsertionEffect:fs,useLayoutEffect:ps,useMemo:G,useReducer:Io,useRef:os,useState:function(){return Io(Fo)},useDebugValue:gs,useDeferredValue:function(e,t){return ys(Ao(),uo.memoizedState,e,t)},useTransition:function(){var e=Io(Fo)[0],t=Ao().memoizedState;return[typeof e==`boolean`?e:Mo(e),t]},useSyncExternalStore:zo,useId:Ts,useHostTransitionStatus:ws,useFormState:ts,useActionState:ts,useOptimistic:function(e,t){return Ko(Ao(),uo,e,t)},useMemoCache:Po,useCacheRefresh:Es};Is.useEffectEvent=ds;var Ls={readContext:Qi,use:No,useCallback:_s,useContext:Qi,useEffect:ls,useImperativeHandle:hs,useInsertionEffect:fs,useLayoutEffect:ps,useMemo:G,useReducer:Ro,useRef:os,useState:function(){return Ro(Fo)},useDebugValue:gs,useDeferredValue:function(e,t){var n=Ao();return uo===null?vs(n,e,t):ys(n,uo.memoizedState,e,t)},useTransition:function(){var e=Ro(Fo)[0],t=Ao().memoizedState;return[typeof e==`boolean`?e:Mo(e),t]},useSyncExternalStore:zo,useId:Ts,useHostTransitionStatus:ws,useFormState:is,useActionState:is,useOptimistic:function(e,t){var n=Ao();return uo===null?(n.baseState=e,[e,n.queue.dispatch]):Ko(n,uo,e,t)},useMemoCache:Po,useCacheRefresh:Es};Ls.useEffectEvent=ds;function Rs(e,t,n,r){t=e.memoizedState,n=n(r,t),n=n==null?t:m({},t,n),e.memoizedState=n,e.lanes===0&&(e.updateQueue.baseState=n)}var zs={enqueueSetState:function(e,t,n){e=e._reactInternals;var r=pu(),i=Ba(r);i.payload=t,n!=null&&(i.callback=n),t=Va(e,i,r),t!==null&&(hu(t,e,r),Ha(t,e,r))},enqueueReplaceState:function(e,t,n){e=e._reactInternals;var r=pu(),i=Ba(r);i.tag=1,i.payload=t,n!=null&&(i.callback=n),t=Va(e,i,r),t!==null&&(hu(t,e,r),Ha(t,e,r))},enqueueForceUpdate:function(e,t){e=e._reactInternals;var n=pu(),r=Ba(n);r.tag=2,t!=null&&(r.callback=t),t=Va(e,r,n),t!==null&&(hu(t,e,n),Ha(t,e,n))}};function Bs(e,t,n,r,i,a,o){return e=e.stateNode,typeof e.shouldComponentUpdate==`function`?e.shouldComponentUpdate(r,a,o):t.prototype&&t.prototype.isPureReactComponent?!Sr(n,r)||!Sr(i,a):!0}function Vs(e,t,n,r){e=t.state,typeof t.componentWillReceiveProps==`function`&&t.componentWillReceiveProps(n,r),typeof t.UNSAFE_componentWillReceiveProps==`function`&&t.UNSAFE_componentWillReceiveProps(n,r),t.state!==e&&zs.enqueueReplaceState(t,t.state,null)}function Hs(e,t){var n=t;if(`ref`in t)for(var r in n={},t)r!==`ref`&&(n[r]=t[r]);if(e=e.defaultProps)for(var i in n===t&&(n=m({},n)),e)n[i]===void 0&&(n[i]=e[i]);return n}function Us(e){qr(e)}function Ws(e){console.error(e)}function Gs(e){qr(e)}function Ks(e,t){try{var n=e.onUncaughtError;n(t.value,{componentStack:t.stack})}catch(e){setTimeout(function(){throw e})}}function qs(e,t,n){try{var r=e.onCaughtError;r(n.value,{componentStack:n.stack,errorBoundary:t.tag===1?t.stateNode:null})}catch(e){setTimeout(function(){throw e})}}function Js(e,t,n){return n=Ba(n),n.tag=3,n.payload={element:null},n.callback=function(){Ks(e,t)},n}function Ys(e){return e=Ba(e),e.tag=3,e}function Xs(e,t,n,r){var i=n.type.getDerivedStateFromError;if(typeof i==`function`){var a=r.value;e.payload=function(){return i(a)},e.callback=function(){qs(t,n,r)}}var o=n.stateNode;o!==null&&typeof o.componentDidCatch==`function`&&(e.callback=function(){qs(t,n,r),typeof i!=`function`&&(ru===null?ru=new Set([this]):ru.add(this));var e=r.stack;this.componentDidCatch(r.value,{componentStack:e===null?``:e})})}function J(e,t,n,r,i){if(n.flags|=32768,typeof r==`object`&&r&&typeof r.then==`function`){if(t=n.alternate,t!==null&&Yi(t,n,i,!0),n=eo.current,n!==null){switch(n.tag){case 31:case 13:return to===null?Du():n.alternate===null&&Wl===0&&(Wl=3),n.flags&=-257,n.flags|=65536,n.lanes=i,r===Sa?n.flags|=16384:(t=n.updateQueue,t===null?n.updateQueue=new Set([r]):t.add(r),Gu(e,r,i)),!1;case 22:return n.flags|=65536,r===Sa?n.flags|=16384:(t=n.updateQueue,t===null?(t={transitions:null,markerInstances:null,retryQueue:new Set([r])},n.updateQueue=t):(n=t.retryQueue,n===null?t.retryQueue=new Set([r]):n.add(r)),Gu(e,r,i)),!1}throw Error(a(435,n.tag))}return Gu(e,r,i),Du(),!1}if(H)return t=eo.current,t===null?(r!==Pi&&(t=Error(a(423),{cause:r}),Vi(hi(t,n))),e=e.current.alternate,e.flags|=65536,i&=-i,e.lanes|=i,r=hi(r,n),i=Js(e.stateNode,r,i),Ua(e,i),Wl!==4&&(Wl=2)):(!(t.flags&65536)&&(t.flags|=256),t.flags|=65536,t.lanes=i,r!==Pi&&(e=Error(a(422),{cause:r}),Vi(hi(e,n)))),!1;var o=Error(a(520),{cause:r});if(o=hi(o,n),Xl===null?Xl=[o]:Xl.push(o),Wl!==4&&(Wl=2),t===null)return!0;r=hi(r,n),n=t;do{switch(n.tag){case 3:return n.flags|=65536,e=i&-i,n.lanes|=e,e=Js(n.stateNode,r,e),Ua(n,e),!1;case 1:if(t=n.type,o=n.stateNode,!(n.flags&128)&&(typeof t.getDerivedStateFromError==`function`||o!==null&&typeof o.componentDidCatch==`function`&&(ru===null||!ru.has(o))))return n.flags|=65536,i&=-i,n.lanes|=i,i=Ys(i),Xs(i,e,n,r),Ua(n,i),!1}n=n.return}while(n!==null);return!1}var Zs=Error(a(461)),Qs=!1;function $s(e,t,n,r){t.child=e===null?Ia(t,null,n,r):Fa(t,e.child,n,r)}function ec(e,t,n,r,i){n=n.render;var a=t.ref;if(`ref`in r){var o={};for(var s in r)s!==`ref`&&(o[s]=r[s])}else o=r;return Zi(t),r=So(e,t,n,o,a,i),s=Eo(),e!==null&&!Qs?(Do(e,t,i),wc(e,t,i)):(H&&s&&Di(t),t.flags|=1,$s(e,t,r,i),t.child)}function tc(e,t,n,r,i){if(e===null){var a=n.type;return typeof a==`function`&&!oi(a)&&a.defaultProps===void 0&&n.compare===null?(t.tag=15,t.type=a,nc(e,t,a,r,i)):(e=li(n.type,null,r,t,t.mode,i),e.ref=t.ref,e.return=t,t.child=e)}if(a=e.child,!Tc(e,i)){var o=a.memoizedProps;if(n=n.compare,n=n===null?Sr:n,n(o,r)&&e.ref===t.ref)return wc(e,t,i)}return t.flags|=1,e=si(a,r),e.ref=t.ref,e.return=t,t.child=e}function nc(e,t,n,r,i){if(e!==null){var a=e.memoizedProps;if(Sr(a,r)&&e.ref===t.ref)if(Qs=!1,t.pendingProps=r=a,Tc(e,i))e.flags&131072&&(Qs=!0);else return t.lanes=e.lanes,wc(e,t,i)}return uc(e,t,n,r,i)}function rc(e,t,n,r){var i=r.children,a=e===null?null:e.memoizedState;if(e===null&&t.stateNode===null&&(t.stateNode={_visibility:1,_pendingMarkers:null,_retryCache:null,_transitions:null}),r.mode===`hidden`){if(t.flags&128){if(a=a===null?n:a.baseLanes|n,e!==null){for(r=t.child=e.child,i=0;r!==null;)i=i|r.lanes|r.childLanes,r=r.sibling;r=i&~a}else r=0,t.child=null;return ac(e,t,a,n,r)}if(n&536870912)t.memoizedState={baseLanes:0,cachePool:null},e!==null&&_a(t,a===null?null:a.cachePool),a===null?Qa():Za(t,a),io(t);else return r=t.lanes=536870912,ac(e,t,a===null?n:a.baseLanes|n,n,r)}else a===null?(e!==null&&_a(t,null),Qa(),ao(t)):(_a(t,a.cachePool),Za(t,a),ao(t),t.memoizedState=null);return $s(e,t,i,n),t.child}function ic(e,t){return e!==null&&e.tag===22||t.stateNode!==null||(t.stateNode={_visibility:1,_pendingMarkers:null,_retryCache:null,_transitions:null}),t.sibling}function ac(e,t,n,r,i){var a=ga();return a=a===null?null:{parent:ia._currentValue,pool:a},t.memoizedState={baseLanes:n,cachePool:a},e!==null&&_a(t,null),Qa(),io(t),e!==null&&Yi(e,t,r,!0),t.childLanes=i,null}function oc(e,t){return t=yc({mode:t.mode,children:t.children},e.mode),t.ref=e.ref,e.child=t,t.return=e,t}function sc(e,t,n){return Fa(t,e.child,null,n),e=oc(t,t.pendingProps),e.flags|=2,oo(t),t.memoizedState=null,e}function cc(e,t,n){var r=t.pendingProps,i=(t.flags&128)!=0;if(t.flags&=-129,e===null){if(H){if(r.mode===`hidden`)return e=oc(t,r),t.lanes=536870912,ic(null,e);if(ro(t),(e=ji)?(e=rf(e,Ni),e=e!==null&&e.data===`&`?e:null,e!==null&&(t.memoizedState={dehydrated:e,treeContext:Si===null?null:{id:Ci,overflow:wi},retryLane:536870912,hydrationErrors:null},n=fi(e),n.return=t,t.child=n,Ai=t,ji=null)):e=null,e===null)throw Fi(t);return t.lanes=536870912,null}return oc(t,r)}var o=e.memoizedState;if(o!==null){var s=o.dehydrated;if(ro(t),i)if(t.flags&256)t.flags&=-257,t=sc(e,t,n);else if(t.memoizedState!==null)t.child=e.child,t.flags|=128,t=null;else throw Error(a(558));else if(Qs||Yi(e,t,n,!1),i=(n&e.childLanes)!==0,Qs||i){if(r=Ll,r!==null&&(s=$e(r,n),s!==0&&s!==o.retryLane))throw o.retryLane=s,ei(e,s),hu(r,e,s),Zs;Du(),t=sc(e,t,n)}else e=o.treeContext,ji=cf(s.nextSibling),Ai=t,H=!0,Mi=null,Ni=!1,e!==null&&ki(t,e),t=oc(t,r),t.flags|=4096;return t}return e=si(e.child,{mode:r.mode,children:r.children}),e.ref=t.ref,t.child=e,e.return=t,e}function lc(e,t){var n=t.ref;if(n===null)e!==null&&e.ref!==null&&(t.flags|=4194816);else{if(typeof n!=`function`&&typeof n!=`object`)throw Error(a(284));(e===null||e.ref!==n)&&(t.flags|=4194816)}}function uc(e,t,n,r,i){return Zi(t),n=So(e,t,n,r,void 0,i),r=Eo(),e!==null&&!Qs?(Do(e,t,i),wc(e,t,i)):(H&&r&&Di(t),t.flags|=1,$s(e,t,n,i),t.child)}function dc(e,t,n,r,i,a){return Zi(t),t.updateQueue=null,n=wo(t,r,n,i),Co(e),r=Eo(),e!==null&&!Qs?(Do(e,t,a),wc(e,t,a)):(H&&r&&Di(t),t.flags|=1,$s(e,t,n,a),t.child)}function fc(e,t,n,r,i){if(Zi(t),t.stateNode===null){var a=ri,o=n.contextType;typeof o==`object`&&o&&(a=Qi(o)),a=new n(r,a),t.memoizedState=a.state!==null&&a.state!==void 0?a.state:null,a.updater=zs,t.stateNode=a,a._reactInternals=t,a=t.stateNode,a.props=r,a.state=t.memoizedState,a.refs={},Ra(t),o=n.contextType,a.context=typeof o==`object`&&o?Qi(o):ri,a.state=t.memoizedState,o=n.getDerivedStateFromProps,typeof o==`function`&&(Rs(t,n,o,r),a.state=t.memoizedState),typeof n.getDerivedStateFromProps==`function`||typeof a.getSnapshotBeforeUpdate==`function`||typeof a.UNSAFE_componentWillMount!=`function`&&typeof a.componentWillMount!=`function`||(o=a.state,typeof a.componentWillMount==`function`&&a.componentWillMount(),typeof a.UNSAFE_componentWillMount==`function`&&a.UNSAFE_componentWillMount(),o!==a.state&&zs.enqueueReplaceState(a,a.state,null),Ka(t,r,a,i),Ga(),a.state=t.memoizedState),typeof a.componentDidMount==`function`&&(t.flags|=4194308),r=!0}else if(e===null){a=t.stateNode;var s=t.memoizedProps,c=Hs(n,s);a.props=c;var l=a.context,u=n.contextType;o=ri,typeof u==`object`&&u&&(o=Qi(u));var d=n.getDerivedStateFromProps;u=typeof d==`function`||typeof a.getSnapshotBeforeUpdate==`function`,s=t.pendingProps!==s,u||typeof a.UNSAFE_componentWillReceiveProps!=`function`&&typeof a.componentWillReceiveProps!=`function`||(s||l!==o)&&Vs(t,a,r,o),La=!1;var f=t.memoizedState;a.state=f,Ka(t,r,a,i),Ga(),l=t.memoizedState,s||f!==l||La?(typeof d==`function`&&(Rs(t,n,d,r),l=t.memoizedState),(c=La||Bs(t,n,c,r,f,l,o))?(u||typeof a.UNSAFE_componentWillMount!=`function`&&typeof a.componentWillMount!=`function`||(typeof a.componentWillMount==`function`&&a.componentWillMount(),typeof a.UNSAFE_componentWillMount==`function`&&a.UNSAFE_componentWillMount()),typeof a.componentDidMount==`function`&&(t.flags|=4194308)):(typeof a.componentDidMount==`function`&&(t.flags|=4194308),t.memoizedProps=r,t.memoizedState=l),a.props=r,a.state=l,a.context=o,r=c):(typeof a.componentDidMount==`function`&&(t.flags|=4194308),r=!1)}else{a=t.stateNode,za(e,t),o=t.memoizedProps,u=Hs(n,o),a.props=u,d=t.pendingProps,f=a.context,l=n.contextType,c=ri,typeof l==`object`&&l&&(c=Qi(l)),s=n.getDerivedStateFromProps,(l=typeof s==`function`||typeof a.getSnapshotBeforeUpdate==`function`)||typeof a.UNSAFE_componentWillReceiveProps!=`function`&&typeof a.componentWillReceiveProps!=`function`||(o!==d||f!==c)&&Vs(t,a,r,c),La=!1,f=t.memoizedState,a.state=f,Ka(t,r,a,i),Ga();var p=t.memoizedState;o!==d||f!==p||La||e!==null&&e.dependencies!==null&&Xi(e.dependencies)?(typeof s==`function`&&(Rs(t,n,s,r),p=t.memoizedState),(u=La||Bs(t,n,u,r,f,p,c)||e!==null&&e.dependencies!==null&&Xi(e.dependencies))?(l||typeof a.UNSAFE_componentWillUpdate!=`function`&&typeof a.componentWillUpdate!=`function`||(typeof a.componentWillUpdate==`function`&&a.componentWillUpdate(r,p,c),typeof a.UNSAFE_componentWillUpdate==`function`&&a.UNSAFE_componentWillUpdate(r,p,c)),typeof a.componentDidUpdate==`function`&&(t.flags|=4),typeof a.getSnapshotBeforeUpdate==`function`&&(t.flags|=1024)):(typeof a.componentDidUpdate!=`function`||o===e.memoizedProps&&f===e.memoizedState||(t.flags|=4),typeof a.getSnapshotBeforeUpdate!=`function`||o===e.memoizedProps&&f===e.memoizedState||(t.flags|=1024),t.memoizedProps=r,t.memoizedState=p),a.props=r,a.state=p,a.context=c,r=u):(typeof a.componentDidUpdate!=`function`||o===e.memoizedProps&&f===e.memoizedState||(t.flags|=4),typeof a.getSnapshotBeforeUpdate!=`function`||o===e.memoizedProps&&f===e.memoizedState||(t.flags|=1024),r=!1)}return a=r,lc(e,t),r=(t.flags&128)!=0,a||r?(a=t.stateNode,n=r&&typeof n.getDerivedStateFromError!=`function`?null:a.render(),t.flags|=1,e!==null&&r?(t.child=Fa(t,e.child,null,i),t.child=Fa(t,null,n,i)):$s(e,t,n,i),t.memoizedState=a.state,e=t.child):e=wc(e,t,i),e}function pc(e,t,n,r){return zi(),t.flags|=256,$s(e,t,n,r),t.child}var mc={dehydrated:null,treeContext:null,retryLane:0,hydrationErrors:null};function hc(e){return{baseLanes:e,cachePool:va()}}function gc(e,t,n){return e=e===null?0:e.childLanes&~n,t&&(e|=Jl),e}function _c(e,t,n){var r=t.pendingProps,i=!1,o=(t.flags&128)!=0,s;if((s=o)||(s=e!==null&&e.memoizedState===null?!1:(so.current&2)!=0),s&&(i=!0,t.flags&=-129),s=(t.flags&32)!=0,t.flags&=-33,e===null){if(H){if(i?no(t):ao(t),(e=ji)?(e=rf(e,Ni),e=e!==null&&e.data!==`&`?e:null,e!==null&&(t.memoizedState={dehydrated:e,treeContext:Si===null?null:{id:Ci,overflow:wi},retryLane:536870912,hydrationErrors:null},n=fi(e),n.return=t,t.child=n,Ai=t,ji=null)):e=null,e===null)throw Fi(t);return of(e)?t.lanes=32:t.lanes=536870912,null}var c=r.children;return r=r.fallback,i?(ao(t),i=t.mode,c=yc({mode:`hidden`,children:c},i),r=ui(r,i,n,null),c.return=t,r.return=t,c.sibling=r,t.child=c,r=t.child,r.memoizedState=hc(n),r.childLanes=gc(e,s,n),t.memoizedState=mc,ic(null,r)):(no(t),vc(t,c))}var l=e.memoizedState;if(l!==null&&(c=l.dehydrated,c!==null)){if(o)t.flags&256?(no(t),t.flags&=-257,t=bc(e,t,n)):t.memoizedState===null?(ao(t),c=r.fallback,i=t.mode,r=yc({mode:`visible`,children:r.children},i),c=ui(c,i,n,null),c.flags|=2,r.return=t,c.return=t,r.sibling=c,t.child=r,Fa(t,e.child,null,n),r=t.child,r.memoizedState=hc(n),r.childLanes=gc(e,s,n),t.memoizedState=mc,t=ic(null,r)):(ao(t),t.child=e.child,t.flags|=128,t=null);else if(no(t),of(c)){if(s=c.nextSibling&&c.nextSibling.dataset,s)var u=s.dgst;s=u,r=Error(a(419)),r.stack=``,r.digest=s,Vi({value:r,source:null,stack:null}),t=bc(e,t,n)}else if(Qs||Yi(e,t,n,!1),s=(n&e.childLanes)!==0,Qs||s){if(s=Ll,s!==null&&(r=$e(s,n),r!==0&&r!==l.retryLane))throw l.retryLane=r,ei(e,r),hu(s,e,r),Zs;af(c)||Du(),t=bc(e,t,n)}else af(c)?(t.flags|=192,t.child=e.child,t=null):(e=l.treeContext,ji=cf(c.nextSibling),Ai=t,H=!0,Mi=null,Ni=!1,e!==null&&ki(t,e),t=vc(t,r.children),t.flags|=4096);return t}return i?(ao(t),c=r.fallback,i=t.mode,l=e.child,u=l.sibling,r=si(l,{mode:`hidden`,children:r.children}),r.subtreeFlags=l.subtreeFlags&65011712,u===null?(c=ui(c,i,n,null),c.flags|=2):c=si(u,c),c.return=t,r.return=t,r.sibling=c,t.child=r,ic(null,r),r=t.child,c=e.child.memoizedState,c===null?c=hc(n):(i=c.cachePool,i===null?i=va():(l=ia._currentValue,i=i.parent===l?i:{parent:l,pool:l}),c={baseLanes:c.baseLanes|n,cachePool:i}),r.memoizedState=c,r.childLanes=gc(e,s,n),t.memoizedState=mc,ic(e.child,r)):(no(t),n=e.child,e=n.sibling,n=si(n,{mode:`visible`,children:r.children}),n.return=t,n.sibling=null,e!==null&&(s=t.deletions,s===null?(t.deletions=[e],t.flags|=16):s.push(e)),t.child=n,t.memoizedState=null,n)}function vc(e,t){return t=yc({mode:`visible`,children:t},e.mode),t.return=e,e.child=t}function yc(e,t){return e=ai(22,e,null,t),e.lanes=0,e}function bc(e,t,n){return Fa(t,e.child,null,n),e=vc(t,t.pendingProps.children),e.flags|=2,t.memoizedState=null,e}function xc(e,t,n){e.lanes|=t;var r=e.alternate;r!==null&&(r.lanes|=t),qi(e.return,t,n)}function Sc(e,t,n,r,i,a){var o=e.memoizedState;o===null?e.memoizedState={isBackwards:t,rendering:null,renderingStartTime:0,last:r,tail:n,tailMode:i,treeForkCount:a}:(o.isBackwards=t,o.rendering=null,o.renderingStartTime=0,o.last=r,o.tail=n,o.tailMode=i,o.treeForkCount=a)}function Cc(e,t,n){var r=t.pendingProps,i=r.revealOrder,a=r.tail;r=r.children;var o=so.current,s=(o&2)!=0;if(s?(o=o&1|2,t.flags|=128):o&=1,L(so,o),$s(e,t,r,n),r=H?yi:0,!s&&e!==null&&e.flags&128)a:for(e=t.child;e!==null;){if(e.tag===13)e.memoizedState!==null&&xc(e,n,t);else if(e.tag===19)xc(e,n,t);else if(e.child!==null){e.child.return=e,e=e.child;continue}if(e===t)break a;for(;e.sibling===null;){if(e.return===null||e.return===t)break a;e=e.return}e.sibling.return=e.return,e=e.sibling}switch(i){case`forwards`:for(n=t.child,i=null;n!==null;)e=n.alternate,e!==null&&co(e)===null&&(i=n),n=n.sibling;n=i,n===null?(i=t.child,t.child=null):(i=n.sibling,n.sibling=null),Sc(t,!1,i,n,a,r);break;case`backwards`:case`unstable_legacy-backwards`:for(n=null,i=t.child,t.child=null;i!==null;){if(e=i.alternate,e!==null&&co(e)===null){t.child=i;break}e=i.sibling,i.sibling=n,n=i,i=e}Sc(t,!0,n,null,a,r);break;case`together`:Sc(t,!1,null,null,void 0,r);break;default:t.memoizedState=null}return t.child}function wc(e,t,n){if(e!==null&&(t.dependencies=e.dependencies),Gl|=t.lanes,(n&t.childLanes)===0)if(e!==null){if(Yi(e,t,n,!1),(n&t.childLanes)===0)return null}else return null;if(e!==null&&t.child!==e.child)throw Error(a(153));if(t.child!==null){for(e=t.child,n=si(e,e.pendingProps),t.child=n,n.return=t;e.sibling!==null;)e=e.sibling,n=n.sibling=si(e,e.pendingProps),n.return=t;n.sibling=null}return t.child}function Tc(e,t){return(e.lanes&t)===0?(e=e.dependencies,!!(e!==null&&Xi(e))):!0}function Ec(e,t,n){switch(t.tag){case 3:ue(t,t.stateNode.containerInfo),Gi(t,ia,e.memoizedState.cache),zi();break;case 27:case 5:de(t);break;case 4:ue(t,t.stateNode.containerInfo);break;case 10:Gi(t,t.type,t.memoizedProps.value);break;case 31:if(t.memoizedState!==null)return t.flags|=128,ro(t),null;break;case 13:var r=t.memoizedState;if(r!==null)return r.dehydrated===null?(n&t.child.childLanes)===0?(no(t),e=wc(e,t,n),e===null?null:e.sibling):_c(e,t,n):(no(t),t.flags|=128,null);no(t);break;case 19:var i=(e.flags&128)!=0;if(r=(n&t.childLanes)!==0,r||=(Yi(e,t,n,!1),(n&t.childLanes)!==0),i){if(r)return Cc(e,t,n);t.flags|=128}if(i=t.memoizedState,i!==null&&(i.rendering=null,i.tail=null,i.lastEffect=null),L(so,so.current),r)break;return null;case 22:return t.lanes=0,rc(e,t,n,t.pendingProps);case 24:Gi(t,ia,e.memoizedState.cache)}return wc(e,t,n)}function Dc(e,t,n){if(e!==null)if(e.memoizedProps!==t.pendingProps)Qs=!0;else{if(!Tc(e,n)&&!(t.flags&128))return Qs=!1,Ec(e,t,n);Qs=!!(e.flags&131072)}else Qs=!1,H&&t.flags&1048576&&Ei(t,yi,t.index);switch(t.lanes=0,t.tag){case 16:a:{var r=t.pendingProps;if(e=Ta(t.elementType),t.type=e,typeof e==`function`)oi(e)?(r=Hs(e,r),t.tag=1,t=fc(null,t,e,r,n)):(t.tag=0,t=uc(null,t,e,r,n));else{if(e!=null){var i=e.$$typeof;if(i===C){t.tag=11,t=ec(null,t,e,r,n);break a}else if(i===T){t.tag=14,t=tc(null,t,e,r,n);break a}}throw t=ne(e)||e,Error(a(306,t,``))}}return t;case 0:return uc(e,t,t.type,t.pendingProps,n);case 1:return r=t.type,i=Hs(r,t.pendingProps),fc(e,t,r,i,n);case 3:a:{if(ue(t,t.stateNode.containerInfo),e===null)throw Error(a(387));r=t.pendingProps;var o=t.memoizedState;i=o.element,za(e,t),Ka(t,r,null,n);var s=t.memoizedState;if(r=s.cache,Gi(t,ia,r),r!==o.cache&&Ji(t,[ia],n,!0),Ga(),r=s.element,o.isDehydrated)if(o={element:r,isDehydrated:!1,cache:s.cache},t.updateQueue.baseState=o,t.memoizedState=o,t.flags&256){t=pc(e,t,r,n);break a}else if(r!==i){i=hi(Error(a(424)),t),Vi(i),t=pc(e,t,r,n);break a}else{switch(e=t.stateNode.containerInfo,e.nodeType){case 9:e=e.body;break;default:e=e.nodeName===`HTML`?e.ownerDocument.body:e}for(ji=cf(e.firstChild),Ai=t,H=!0,Mi=null,Ni=!0,n=Ia(t,null,r,n),t.child=n;n;)n.flags=n.flags&-3|4096,n=n.sibling}else{if(zi(),r===i){t=wc(e,t,n);break a}$s(e,t,r,n)}t=t.child}return t;case 26:return lc(e,t),e===null?(n=kf(t.type,null,t.pendingProps,null))?t.memoizedState=n:H||(n=t.type,e=t.pendingProps,r=Bd(ce.current).createElement(n),r[at]=t,r[ot]=e,Pd(r,n,e),vt(r),t.stateNode=r):t.memoizedState=kf(t.type,e.memoizedProps,t.pendingProps,e.memoizedState),null;case 27:return de(t),e===null&&H&&(r=t.stateNode=ff(t.type,t.pendingProps,ce.current),Ai=t,Ni=!0,i=ji,Zd(t.type)?(lf=i,ji=cf(r.firstChild)):ji=i),$s(e,t,t.pendingProps.children,n),lc(e,t),e===null&&(t.flags|=4194304),t.child;case 5:return e===null&&H&&((i=r=ji)&&(r=tf(r,t.type,t.pendingProps,Ni),r===null?i=!1:(t.stateNode=r,Ai=t,ji=cf(r.firstChild),Ni=!1,i=!0)),i||Fi(t)),de(t),i=t.type,o=t.pendingProps,s=e===null?null:e.memoizedProps,r=o.children,Ud(i,o)?r=null:s!==null&&Ud(i,s)&&(t.flags|=32),t.memoizedState!==null&&(i=So(e,t,To,null,null,n),Qf._currentValue=i),lc(e,t),$s(e,t,r,n),t.child;case 6:return e===null&&H&&((e=n=ji)&&(n=nf(n,t.pendingProps,Ni),n===null?e=!1:(t.stateNode=n,Ai=t,ji=null,e=!0)),e||Fi(t)),null;case 13:return _c(e,t,n);case 4:return ue(t,t.stateNode.containerInfo),r=t.pendingProps,e===null?t.child=Fa(t,null,r,n):$s(e,t,r,n),t.child;case 11:return ec(e,t,t.type,t.pendingProps,n);case 7:return $s(e,t,t.pendingProps,n),t.child;case 8:return $s(e,t,t.pendingProps.children,n),t.child;case 12:return $s(e,t,t.pendingProps.children,n),t.child;case 10:return r=t.pendingProps,Gi(t,t.type,r.value),$s(e,t,r.children,n),t.child;case 9:return i=t.type._context,r=t.pendingProps.children,Zi(t),i=Qi(i),r=r(i),t.flags|=1,$s(e,t,r,n),t.child;case 14:return tc(e,t,t.type,t.pendingProps,n);case 15:return nc(e,t,t.type,t.pendingProps,n);case 19:return Cc(e,t,n);case 31:return cc(e,t,n);case 22:return rc(e,t,n,t.pendingProps);case 24:return Zi(t),r=Qi(ia),e===null?(i=ga(),i===null&&(i=Ll,o=aa(),i.pooledCache=o,o.refCount++,o!==null&&(i.pooledCacheLanes|=n),i=o),t.memoizedState={parent:r,cache:i},Ra(t),Gi(t,ia,i)):((e.lanes&n)!==0&&(za(e,t),Ka(t,null,null,n),Ga()),i=e.memoizedState,o=t.memoizedState,i.parent===r?(r=o.cache,Gi(t,ia,r),r!==i.cache&&Ji(t,[ia],n,!0)):(i={parent:r,cache:r},t.memoizedState=i,t.lanes===0&&(t.memoizedState=t.updateQueue.baseState=i),Gi(t,ia,r))),$s(e,t,t.pendingProps.children,n),t.child;case 29:throw t.pendingProps}throw Error(a(156,t.tag))}function Oc(e){e.flags|=4}function kc(e,t,n,r,i){if((t=(e.mode&32)!=0)&&(t=!1),t){if(e.flags|=16777216,(i&335544128)===i)if(e.stateNode.complete)e.flags|=8192;else if(wu())e.flags|=8192;else throw Ea=Sa,ba}else e.flags&=-16777217}function Ac(e,t){if(t.type!==`stylesheet`||t.state.loading&4)e.flags&=-16777217;else if(e.flags|=16777216,!Wf(t))if(wu())e.flags|=8192;else throw Ea=Sa,ba}function jc(e,t){t!==null&&(e.flags|=4),e.flags&16384&&(t=e.tag===22?536870912:qe(),e.lanes|=t,Yl|=t)}function Mc(e,t){if(!H)switch(e.tailMode){case`hidden`:t=e.tail;for(var n=null;t!==null;)t.alternate!==null&&(n=t),t=t.sibling;n===null?e.tail=null:n.sibling=null;break;case`collapsed`:n=e.tail;for(var r=null;n!==null;)n.alternate!==null&&(r=n),n=n.sibling;r===null?t||e.tail===null?e.tail=null:e.tail.sibling=null:r.sibling=null}}function Nc(e){var t=e.alternate!==null&&e.alternate.child===e.child,n=0,r=0;if(t)for(var i=e.child;i!==null;)n|=i.lanes|i.childLanes,r|=i.subtreeFlags&65011712,r|=i.flags&65011712,i.return=e,i=i.sibling;else for(i=e.child;i!==null;)n|=i.lanes|i.childLanes,r|=i.subtreeFlags,r|=i.flags,i.return=e,i=i.sibling;return e.subtreeFlags|=r,e.childLanes=n,t}function Pc(e,t,n){var r=t.pendingProps;switch(Oi(t),t.tag){case 16:case 15:case 0:case 11:case 7:case 8:case 12:case 9:case 14:return Nc(t),null;case 1:return Nc(t),null;case 3:return n=t.stateNode,r=null,e!==null&&(r=e.memoizedState.cache),t.memoizedState.cache!==r&&(t.flags|=2048),Ki(ia),R(),n.pendingContext&&(n.context=n.pendingContext,n.pendingContext=null),(e===null||e.child===null)&&(Ri(t)?Oc(t):e===null||e.memoizedState.isDehydrated&&!(t.flags&256)||(t.flags|=1024,Bi())),Nc(t),null;case 26:var i=t.type,o=t.memoizedState;return e===null?(Oc(t),o===null?(Nc(t),kc(t,i,null,r,n)):(Nc(t),Ac(t,o))):o?o===e.memoizedState?(Nc(t),t.flags&=-16777217):(Oc(t),Nc(t),Ac(t,o)):(e=e.memoizedProps,e!==r&&Oc(t),Nc(t),kc(t,i,e,r,n)),null;case 27:if(z(t),n=ce.current,i=t.type,e!==null&&t.stateNode!=null)e.memoizedProps!==r&&Oc(t);else{if(!r){if(t.stateNode===null)throw Error(a(166));return Nc(t),null}e=oe.current,Ri(t)?Ii(t,e):(e=ff(i,r,n),t.stateNode=e,Oc(t))}return Nc(t),null;case 5:if(z(t),i=t.type,e!==null&&t.stateNode!=null)e.memoizedProps!==r&&Oc(t);else{if(!r){if(t.stateNode===null)throw Error(a(166));return Nc(t),null}if(o=oe.current,Ri(t))Ii(t,o);else{var s=Bd(ce.current);switch(o){case 1:o=s.createElementNS(`http://www.w3.org/2000/svg`,i);break;case 2:o=s.createElementNS(`http://www.w3.org/1998/Math/MathML`,i);break;default:switch(i){case`svg`:o=s.createElementNS(`http://www.w3.org/2000/svg`,i);break;case`math`:o=s.createElementNS(`http://www.w3.org/1998/Math/MathML`,i);break;case`script`:o=s.createElement(`div`),o.innerHTML=`<script><\/script>`,o=o.removeChild(o.firstChild);break;case`select`:o=typeof r.is==`string`?s.createElement(`select`,{is:r.is}):s.createElement(`select`),r.multiple?o.multiple=!0:r.size&&(o.size=r.size);break;default:o=typeof r.is==`string`?s.createElement(i,{is:r.is}):s.createElement(i)}}o[at]=t,o[ot]=r;a:for(s=t.child;s!==null;){if(s.tag===5||s.tag===6)o.appendChild(s.stateNode);else if(s.tag!==4&&s.tag!==27&&s.child!==null){s.child.return=s,s=s.child;continue}if(s===t)break a;for(;s.sibling===null;){if(s.return===null||s.return===t)break a;s=s.return}s.sibling.return=s.return,s=s.sibling}t.stateNode=o;a:switch(Pd(o,i,r),i){case`button`:case`input`:case`select`:case`textarea`:r=!!r.autoFocus;break a;case`img`:r=!0;break a;default:r=!1}r&&Oc(t)}}return Nc(t),kc(t,t.type,e===null?null:e.memoizedProps,t.pendingProps,n),null;case 6:if(e&&t.stateNode!=null)e.memoizedProps!==r&&Oc(t);else{if(typeof r!=`string`&&t.stateNode===null)throw Error(a(166));if(e=ce.current,Ri(t)){if(e=t.stateNode,n=t.memoizedProps,r=null,i=Ai,i!==null)switch(i.tag){case 27:case 5:r=i.memoizedProps}e[at]=t,e=!!(e.nodeValue===n||r!==null&&!0===r.suppressHydrationWarning||jd(e.nodeValue,n)),e||Fi(t,!0)}else e=Bd(e).createTextNode(r),e[at]=t,t.stateNode=e}return Nc(t),null;case 31:if(n=t.memoizedState,e===null||e.memoizedState!==null){if(r=Ri(t),n!==null){if(e===null){if(!r)throw Error(a(318));if(e=t.memoizedState,e=e===null?null:e.dehydrated,!e)throw Error(a(557));e[at]=t}else zi(),!(t.flags&128)&&(t.memoizedState=null),t.flags|=4;Nc(t),e=!1}else n=Bi(),e!==null&&e.memoizedState!==null&&(e.memoizedState.hydrationErrors=n),e=!0;if(!e)return t.flags&256?(oo(t),t):(oo(t),null);if(t.flags&128)throw Error(a(558))}return Nc(t),null;case 13:if(r=t.memoizedState,e===null||e.memoizedState!==null&&e.memoizedState.dehydrated!==null){if(i=Ri(t),r!==null&&r.dehydrated!==null){if(e===null){if(!i)throw Error(a(318));if(i=t.memoizedState,i=i===null?null:i.dehydrated,!i)throw Error(a(317));i[at]=t}else zi(),!(t.flags&128)&&(t.memoizedState=null),t.flags|=4;Nc(t),i=!1}else i=Bi(),e!==null&&e.memoizedState!==null&&(e.memoizedState.hydrationErrors=i),i=!0;if(!i)return t.flags&256?(oo(t),t):(oo(t),null)}return oo(t),t.flags&128?(t.lanes=n,t):(n=r!==null,e=e!==null&&e.memoizedState!==null,n&&(r=t.child,i=null,r.alternate!==null&&r.alternate.memoizedState!==null&&r.alternate.memoizedState.cachePool!==null&&(i=r.alternate.memoizedState.cachePool.pool),o=null,r.memoizedState!==null&&r.memoizedState.cachePool!==null&&(o=r.memoizedState.cachePool.pool),o!==i&&(r.flags|=2048)),n!==e&&n&&(t.child.flags|=8192),jc(t,t.updateQueue),Nc(t),null);case 4:return R(),e===null&&xd(t.stateNode.containerInfo),Nc(t),null;case 10:return Ki(t.type),Nc(t),null;case 19:if(I(so),r=t.memoizedState,r===null)return Nc(t),null;if(i=(t.flags&128)!=0,o=r.rendering,o===null)if(i)Mc(r,!1);else{if(Wl!==0||e!==null&&e.flags&128)for(e=t.child;e!==null;){if(o=co(e),o!==null){for(t.flags|=128,Mc(r,!1),e=o.updateQueue,t.updateQueue=e,jc(t,e),t.subtreeFlags=0,e=n,n=t.child;n!==null;)ci(n,e),n=n.sibling;return L(so,so.current&1|2),H&&Ti(t,r.treeForkCount),t.child}e=e.sibling}r.tail!==null&&we()>tu&&(t.flags|=128,i=!0,Mc(r,!1),t.lanes=4194304)}else{if(!i)if(e=co(o),e!==null){if(t.flags|=128,i=!0,e=e.updateQueue,t.updateQueue=e,jc(t,e),Mc(r,!0),r.tail===null&&r.tailMode===`hidden`&&!o.alternate&&!H)return Nc(t),null}else 2*we()-r.renderingStartTime>tu&&n!==536870912&&(t.flags|=128,i=!0,Mc(r,!1),t.lanes=4194304);r.isBackwards?(o.sibling=t.child,t.child=o):(e=r.last,e===null?t.child=o:e.sibling=o,r.last=o)}return r.tail===null?(Nc(t),null):(e=r.tail,r.rendering=e,r.tail=e.sibling,r.renderingStartTime=we(),e.sibling=null,n=so.current,L(so,i?n&1|2:n&1),H&&Ti(t,r.treeForkCount),e);case 22:case 23:return oo(t),$a(),r=t.memoizedState!==null,e===null?r&&(t.flags|=8192):e.memoizedState!==null!==r&&(t.flags|=8192),r?n&536870912&&!(t.flags&128)&&(Nc(t),t.subtreeFlags&6&&(t.flags|=8192)):Nc(t),n=t.updateQueue,n!==null&&jc(t,n.retryQueue),n=null,e!==null&&e.memoizedState!==null&&e.memoizedState.cachePool!==null&&(n=e.memoizedState.cachePool.pool),r=null,t.memoizedState!==null&&t.memoizedState.cachePool!==null&&(r=t.memoizedState.cachePool.pool),r!==n&&(t.flags|=2048),e!==null&&I(ha),null;case 24:return n=null,e!==null&&(n=e.memoizedState.cache),t.memoizedState.cache!==n&&(t.flags|=2048),Ki(ia),Nc(t),null;case 25:return null;case 30:return null}throw Error(a(156,t.tag))}function Fc(e,t){switch(Oi(t),t.tag){case 1:return e=t.flags,e&65536?(t.flags=e&-65537|128,t):null;case 3:return Ki(ia),R(),e=t.flags,e&65536&&!(e&128)?(t.flags=e&-65537|128,t):null;case 26:case 27:case 5:return z(t),null;case 31:if(t.memoizedState!==null){if(oo(t),t.alternate===null)throw Error(a(340));zi()}return e=t.flags,e&65536?(t.flags=e&-65537|128,t):null;case 13:if(oo(t),e=t.memoizedState,e!==null&&e.dehydrated!==null){if(t.alternate===null)throw Error(a(340));zi()}return e=t.flags,e&65536?(t.flags=e&-65537|128,t):null;case 19:return I(so),null;case 4:return R(),null;case 10:return Ki(t.type),null;case 22:case 23:return oo(t),$a(),e!==null&&I(ha),e=t.flags,e&65536?(t.flags=e&-65537|128,t):null;case 24:return Ki(ia),null;case 25:return null;default:return null}}function Ic(e,t){switch(Oi(t),t.tag){case 3:Ki(ia),R();break;case 26:case 27:case 5:z(t);break;case 4:R();break;case 31:t.memoizedState!==null&&oo(t);break;case 13:oo(t);break;case 19:I(so);break;case 10:Ki(t.type);break;case 22:case 23:oo(t),$a(),e!==null&&I(ha);break;case 24:Ki(ia)}}function Lc(e,t){try{var n=t.updateQueue,r=n===null?null:n.lastEffect;if(r!==null){var i=r.next;n=i;do{if((n.tag&e)===e){r=void 0;var a=n.create,o=n.inst;r=a(),o.destroy=r}n=n.next}while(n!==i)}}catch(e){Z(t,t.return,e)}}function Rc(e,t,n){try{var r=t.updateQueue,i=r===null?null:r.lastEffect;if(i!==null){var a=i.next;r=a;do{if((r.tag&e)===e){var o=r.inst,s=o.destroy;if(s!==void 0){o.destroy=void 0,i=t;var c=n,l=s;try{l()}catch(e){Z(i,c,e)}}}r=r.next}while(r!==a)}}catch(e){Z(t,t.return,e)}}function zc(e){var t=e.updateQueue;if(t!==null){var n=e.stateNode;try{Ja(t,n)}catch(t){Z(e,e.return,t)}}}function Bc(e,t,n){n.props=Hs(e.type,e.memoizedProps),n.state=e.memoizedState;try{n.componentWillUnmount()}catch(n){Z(e,t,n)}}function Vc(e,t){try{var n=e.ref;if(n!==null){switch(e.tag){case 26:case 27:case 5:var r=e.stateNode;break;case 30:r=e.stateNode;break;default:r=e.stateNode}typeof n==`function`?e.refCleanup=n(r):n.current=r}}catch(n){Z(e,t,n)}}function Hc(e,t){var n=e.ref,r=e.refCleanup;if(n!==null)if(typeof r==`function`)try{r()}catch(n){Z(e,t,n)}finally{e.refCleanup=null,e=e.alternate,e!=null&&(e.refCleanup=null)}else if(typeof n==`function`)try{n(null)}catch(n){Z(e,t,n)}else n.current=null}function Uc(e){var t=e.type,n=e.memoizedProps,r=e.stateNode;try{a:switch(t){case`button`:case`input`:case`select`:case`textarea`:n.autoFocus&&r.focus();break a;case`img`:n.src?r.src=n.src:n.srcSet&&(r.srcset=n.srcSet)}}catch(t){Z(e,e.return,t)}}function Wc(e,t,n){try{var r=e.stateNode;Fd(r,e.type,n,t),r[ot]=t}catch(t){Z(e,e.return,t)}}function Gc(e){return e.tag===5||e.tag===3||e.tag===26||e.tag===27&&Zd(e.type)||e.tag===4}function Kc(e){a:for(;;){for(;e.sibling===null;){if(e.return===null||Gc(e.return))return null;e=e.return}for(e.sibling.return=e.return,e=e.sibling;e.tag!==5&&e.tag!==6&&e.tag!==18;){if(e.tag===27&&Zd(e.type)||e.flags&2||e.child===null||e.tag===4)continue a;e.child.return=e,e=e.child}if(!(e.flags&2))return e.stateNode}}function qc(e,t,n){var r=e.tag;if(r===5||r===6)e=e.stateNode,t?(n.nodeType===9?n.body:n.nodeName===`HTML`?n.ownerDocument.body:n).insertBefore(e,t):(t=n.nodeType===9?n.body:n.nodeName===`HTML`?n.ownerDocument.body:n,t.appendChild(e),n=n._reactRootContainer,n!=null||t.onclick!==null||(t.onclick=Qt));else if(r!==4&&(r===27&&Zd(e.type)&&(n=e.stateNode,t=null),e=e.child,e!==null))for(qc(e,t,n),e=e.sibling;e!==null;)qc(e,t,n),e=e.sibling}function Jc(e,t,n){var r=e.tag;if(r===5||r===6)e=e.stateNode,t?n.insertBefore(e,t):n.appendChild(e);else if(r!==4&&(r===27&&Zd(e.type)&&(n=e.stateNode),e=e.child,e!==null))for(Jc(e,t,n),e=e.sibling;e!==null;)Jc(e,t,n),e=e.sibling}function Yc(e){var t=e.stateNode,n=e.memoizedProps;try{for(var r=e.type,i=t.attributes;i.length;)t.removeAttributeNode(i[0]);Pd(t,r,n),t[at]=e,t[ot]=n}catch(t){Z(e,e.return,t)}}var Xc=!1,Zc=!1,Qc=!1,$c=typeof WeakSet==`function`?WeakSet:Set,el=null;function tl(e,t){if(e=e.containerInfo,Rd=sp,e=Er(e),Dr(e)){if(`selectionStart`in e)var n={start:e.selectionStart,end:e.selectionEnd};else a:{n=(n=e.ownerDocument)&&n.defaultView||window;var r=n.getSelection&&n.getSelection();if(r&&r.rangeCount!==0){n=r.anchorNode;var i=r.anchorOffset,o=r.focusNode;r=r.focusOffset;try{n.nodeType,o.nodeType}catch{n=null;break a}var s=0,c=-1,l=-1,u=0,d=0,f=e,p=null;b:for(;;){for(var m;f!==n||i!==0&&f.nodeType!==3||(c=s+i),f!==o||r!==0&&f.nodeType!==3||(l=s+r),f.nodeType===3&&(s+=f.nodeValue.length),(m=f.firstChild)!==null;)p=f,f=m;for(;;){if(f===e)break b;if(p===n&&++u===i&&(c=s),p===o&&++d===r&&(l=s),(m=f.nextSibling)!==null)break;f=p,p=f.parentNode}f=m}n=c===-1||l===-1?null:{start:c,end:l}}else n=null}n||={start:0,end:0}}else n=null;for(zd={focusedElem:e,selectionRange:n},sp=!1,el=t;el!==null;)if(t=el,e=t.child,t.subtreeFlags&1028&&e!==null)e.return=t,el=e;else for(;el!==null;){switch(t=el,o=t.alternate,e=t.flags,t.tag){case 0:if(e&4&&(e=t.updateQueue,e=e===null?null:e.events,e!==null))for(n=0;n<e.length;n++)i=e[n],i.ref.impl=i.nextImpl;break;case 11:case 15:break;case 1:if(e&1024&&o!==null){e=void 0,n=t,i=o.memoizedProps,o=o.memoizedState,r=n.stateNode;try{var h=Hs(n.type,i);e=r.getSnapshotBeforeUpdate(h,o),r.__reactInternalSnapshotBeforeUpdate=e}catch(e){Z(n,n.return,e)}}break;case 3:if(e&1024){if(e=t.stateNode.containerInfo,n=e.nodeType,n===9)ef(e);else if(n===1)switch(e.nodeName){case`HEAD`:case`HTML`:case`BODY`:ef(e);break;default:e.textContent=``}}break;case 5:case 26:case 27:case 6:case 4:case 17:break;default:if(e&1024)throw Error(a(163))}if(e=t.sibling,e!==null){e.return=t.return,el=e;break}el=t.return}}function nl(e,t,n){var r=n.flags;switch(n.tag){case 0:case 11:case 15:_l(e,n),r&4&&Lc(5,n);break;case 1:if(_l(e,n),r&4)if(e=n.stateNode,t===null)try{e.componentDidMount()}catch(e){Z(n,n.return,e)}else{var i=Hs(n.type,t.memoizedProps);t=t.memoizedState;try{e.componentDidUpdate(i,t,e.__reactInternalSnapshotBeforeUpdate)}catch(e){Z(n,n.return,e)}}r&64&&zc(n),r&512&&Vc(n,n.return);break;case 3:if(_l(e,n),r&64&&(e=n.updateQueue,e!==null)){if(t=null,n.child!==null)switch(n.child.tag){case 27:case 5:t=n.child.stateNode;break;case 1:t=n.child.stateNode}try{Ja(e,t)}catch(e){Z(n,n.return,e)}}break;case 27:t===null&&r&4&&Yc(n);case 26:case 5:_l(e,n),t===null&&r&4&&Uc(n),r&512&&Vc(n,n.return);break;case 12:_l(e,n);break;case 31:_l(e,n),r&4&&cl(e,n);break;case 13:_l(e,n),r&4&&ll(e,n),r&64&&(e=n.memoizedState,e!==null&&(e=e.dehydrated,e!==null&&(n=Ju.bind(null,n),sf(e,n))));break;case 22:if(r=n.memoizedState!==null||Xc,!r){t=t!==null&&t.memoizedState!==null||Zc,i=Xc;var a=Zc;Xc=r,(Zc=t)&&!a?yl(e,n,(n.subtreeFlags&8772)!=0):_l(e,n),Xc=i,Zc=a}break;case 30:break;default:_l(e,n)}}function rl(e){var t=e.alternate;t!==null&&(e.alternate=null,rl(t)),e.child=null,e.deletions=null,e.sibling=null,e.tag===5&&(t=e.stateNode,t!==null&&pt(t)),e.stateNode=null,e.return=null,e.dependencies=null,e.memoizedProps=null,e.memoizedState=null,e.pendingProps=null,e.stateNode=null,e.updateQueue=null}var il=null,al=!1;function ol(e,t,n){for(n=n.child;n!==null;)sl(e,t,n),n=n.sibling}function sl(e,t,n){if(Pe&&typeof Pe.onCommitFiberUnmount==`function`)try{Pe.onCommitFiberUnmount(Ne,n)}catch{}switch(n.tag){case 26:Zc||Hc(n,t),ol(e,t,n),n.memoizedState?n.memoizedState.count--:n.stateNode&&(n=n.stateNode,n.parentNode.removeChild(n));break;case 27:Zc||Hc(n,t);var r=il,i=al;Zd(n.type)&&(il=n.stateNode,al=!1),ol(e,t,n),pf(n.stateNode),il=r,al=i;break;case 5:Zc||Hc(n,t);case 6:if(r=il,i=al,il=null,ol(e,t,n),il=r,al=i,il!==null)if(al)try{(il.nodeType===9?il.body:il.nodeName===`HTML`?il.ownerDocument.body:il).removeChild(n.stateNode)}catch(e){Z(n,t,e)}else try{il.removeChild(n.stateNode)}catch(e){Z(n,t,e)}break;case 18:il!==null&&(al?(e=il,Qd(e.nodeType===9?e.body:e.nodeName===`HTML`?e.ownerDocument.body:e,n.stateNode),Np(e)):Qd(il,n.stateNode));break;case 4:r=il,i=al,il=n.stateNode.containerInfo,al=!0,ol(e,t,n),il=r,al=i;break;case 0:case 11:case 14:case 15:Rc(2,n,t),Zc||Rc(4,n,t),ol(e,t,n);break;case 1:Zc||(Hc(n,t),r=n.stateNode,typeof r.componentWillUnmount==`function`&&Bc(n,t,r)),ol(e,t,n);break;case 21:ol(e,t,n);break;case 22:Zc=(r=Zc)||n.memoizedState!==null,ol(e,t,n),Zc=r;break;default:ol(e,t,n)}}function cl(e,t){if(t.memoizedState===null&&(e=t.alternate,e!==null&&(e=e.memoizedState,e!==null))){e=e.dehydrated;try{Np(e)}catch(e){Z(t,t.return,e)}}}function ll(e,t){if(t.memoizedState===null&&(e=t.alternate,e!==null&&(e=e.memoizedState,e!==null&&(e=e.dehydrated,e!==null))))try{Np(e)}catch(e){Z(t,t.return,e)}}function ul(e){switch(e.tag){case 31:case 13:case 19:var t=e.stateNode;return t===null&&(t=e.stateNode=new $c),t;case 22:return e=e.stateNode,t=e._retryCache,t===null&&(t=e._retryCache=new $c),t;default:throw Error(a(435,e.tag))}}function dl(e,t){var n=ul(e);t.forEach(function(t){if(!n.has(t)){n.add(t);var r=Yu.bind(null,e,t);t.then(r,r)}})}function fl(e,t){var n=t.deletions;if(n!==null)for(var r=0;r<n.length;r++){var i=n[r],o=e,s=t,c=s;a:for(;c!==null;){switch(c.tag){case 27:if(Zd(c.type)){il=c.stateNode,al=!1;break a}break;case 5:il=c.stateNode,al=!1;break a;case 3:case 4:il=c.stateNode.containerInfo,al=!0;break a}c=c.return}if(il===null)throw Error(a(160));sl(o,s,i),il=null,al=!1,o=i.alternate,o!==null&&(o.return=null),i.return=null}if(t.subtreeFlags&13886)for(t=t.child;t!==null;)ml(t,e),t=t.sibling}var pl=null;function ml(e,t){var n=e.alternate,r=e.flags;switch(e.tag){case 0:case 11:case 14:case 15:fl(t,e),hl(e),r&4&&(Rc(3,e,e.return),Lc(3,e),Rc(5,e,e.return));break;case 1:fl(t,e),hl(e),r&512&&(Zc||n===null||Hc(n,n.return)),r&64&&Xc&&(e=e.updateQueue,e!==null&&(r=e.callbacks,r!==null&&(n=e.shared.hiddenCallbacks,e.shared.hiddenCallbacks=n===null?r:n.concat(r))));break;case 26:var i=pl;if(fl(t,e),hl(e),r&512&&(Zc||n===null||Hc(n,n.return)),r&4){var o=n===null?null:n.memoizedState;if(r=e.memoizedState,n===null)if(r===null)if(e.stateNode===null){a:{r=e.type,n=e.memoizedProps,i=i.ownerDocument||i;b:switch(r){case`title`:o=i.getElementsByTagName(`title`)[0],(!o||o[ft]||o[at]||o.namespaceURI===`http://www.w3.org/2000/svg`||o.hasAttribute(`itemprop`))&&(o=i.createElement(r),i.head.insertBefore(o,i.querySelector(`head > title`))),Pd(o,r,n),o[at]=e,vt(o),r=o;break a;case`link`:var s=Vf(`link`,`href`,i).get(r+(n.href||``));if(s){for(var c=0;c<s.length;c++)if(o=s[c],o.getAttribute(`href`)===(n.href==null||n.href===``?null:n.href)&&o.getAttribute(`rel`)===(n.rel==null?null:n.rel)&&o.getAttribute(`title`)===(n.title==null?null:n.title)&&o.getAttribute(`crossorigin`)===(n.crossOrigin==null?null:n.crossOrigin)){s.splice(c,1);break b}}o=i.createElement(r),Pd(o,r,n),i.head.appendChild(o);break;case`meta`:if(s=Vf(`meta`,`content`,i).get(r+(n.content||``))){for(c=0;c<s.length;c++)if(o=s[c],o.getAttribute(`content`)===(n.content==null?null:``+n.content)&&o.getAttribute(`name`)===(n.name==null?null:n.name)&&o.getAttribute(`property`)===(n.property==null?null:n.property)&&o.getAttribute(`http-equiv`)===(n.httpEquiv==null?null:n.httpEquiv)&&o.getAttribute(`charset`)===(n.charSet==null?null:n.charSet)){s.splice(c,1);break b}}o=i.createElement(r),Pd(o,r,n),i.head.appendChild(o);break;default:throw Error(a(468,r))}o[at]=e,vt(o),r=o}e.stateNode=r}else Hf(i,e.type,e.stateNode);else e.stateNode=If(i,r,e.memoizedProps);else o===r?r===null&&e.stateNode!==null&&Wc(e,e.memoizedProps,n.memoizedProps):(o===null?n.stateNode!==null&&(n=n.stateNode,n.parentNode.removeChild(n)):o.count--,r===null?Hf(i,e.type,e.stateNode):If(i,r,e.memoizedProps))}break;case 27:fl(t,e),hl(e),r&512&&(Zc||n===null||Hc(n,n.return)),n!==null&&r&4&&Wc(e,e.memoizedProps,n.memoizedProps);break;case 5:if(fl(t,e),hl(e),r&512&&(Zc||n===null||Hc(n,n.return)),e.flags&32){i=e.stateNode;try{Wt(i,``)}catch(t){Z(e,e.return,t)}}r&4&&e.stateNode!=null&&(i=e.memoizedProps,Wc(e,i,n===null?i:n.memoizedProps)),r&1024&&(Qc=!0);break;case 6:if(fl(t,e),hl(e),r&4){if(e.stateNode===null)throw Error(a(162));r=e.memoizedProps,n=e.stateNode;try{n.nodeValue=r}catch(t){Z(e,e.return,t)}}break;case 3:if(Bf=null,i=pl,pl=gf(t.containerInfo),fl(t,e),pl=i,hl(e),r&4&&n!==null&&n.memoizedState.isDehydrated)try{Np(t.containerInfo)}catch(t){Z(e,e.return,t)}Qc&&(Qc=!1,gl(e));break;case 4:r=pl,pl=gf(e.stateNode.containerInfo),fl(t,e),hl(e),pl=r;break;case 12:fl(t,e),hl(e);break;case 31:fl(t,e),hl(e),r&4&&(r=e.updateQueue,r!==null&&(e.updateQueue=null,dl(e,r)));break;case 13:fl(t,e),hl(e),e.child.flags&8192&&e.memoizedState!==null!=(n!==null&&n.memoizedState!==null)&&($l=we()),r&4&&(r=e.updateQueue,r!==null&&(e.updateQueue=null,dl(e,r)));break;case 22:i=e.memoizedState!==null;var l=n!==null&&n.memoizedState!==null,u=Xc,d=Zc;if(Xc=u||i,Zc=d||l,fl(t,e),Zc=d,Xc=u,hl(e),r&8192)a:for(t=e.stateNode,t._visibility=i?t._visibility&-2:t._visibility|1,i&&(n===null||l||Xc||Zc||vl(e)),n=null,t=e;;){if(t.tag===5||t.tag===26){if(n===null){l=n=t;try{if(o=l.stateNode,i)s=o.style,typeof s.setProperty==`function`?s.setProperty(`display`,`none`,`important`):s.display=`none`;else{c=l.stateNode;var f=l.memoizedProps.style,p=f!=null&&f.hasOwnProperty(`display`)?f.display:null;c.style.display=p==null||typeof p==`boolean`?``:(``+p).trim()}}catch(e){Z(l,l.return,e)}}}else if(t.tag===6){if(n===null){l=t;try{l.stateNode.nodeValue=i?``:l.memoizedProps}catch(e){Z(l,l.return,e)}}}else if(t.tag===18){if(n===null){l=t;try{var m=l.stateNode;i?$d(m,!0):$d(l.stateNode,!1)}catch(e){Z(l,l.return,e)}}}else if((t.tag!==22&&t.tag!==23||t.memoizedState===null||t===e)&&t.child!==null){t.child.return=t,t=t.child;continue}if(t===e)break a;for(;t.sibling===null;){if(t.return===null||t.return===e)break a;n===t&&(n=null),t=t.return}n===t&&(n=null),t.sibling.return=t.return,t=t.sibling}r&4&&(r=e.updateQueue,r!==null&&(n=r.retryQueue,n!==null&&(r.retryQueue=null,dl(e,n))));break;case 19:fl(t,e),hl(e),r&4&&(r=e.updateQueue,r!==null&&(e.updateQueue=null,dl(e,r)));break;case 30:break;case 21:break;default:fl(t,e),hl(e)}}function hl(e){var t=e.flags;if(t&2){try{for(var n,r=e.return;r!==null;){if(Gc(r)){n=r;break}r=r.return}if(n==null)throw Error(a(160));switch(n.tag){case 27:var i=n.stateNode;Jc(e,Kc(e),i);break;case 5:var o=n.stateNode;n.flags&32&&(Wt(o,``),n.flags&=-33),Jc(e,Kc(e),o);break;case 3:case 4:var s=n.stateNode.containerInfo;qc(e,Kc(e),s);break;default:throw Error(a(161))}}catch(t){Z(e,e.return,t)}e.flags&=-3}t&4096&&(e.flags&=-4097)}function gl(e){if(e.subtreeFlags&1024)for(e=e.child;e!==null;){var t=e;gl(t),t.tag===5&&t.flags&1024&&t.stateNode.reset(),e=e.sibling}}function _l(e,t){if(t.subtreeFlags&8772)for(t=t.child;t!==null;)nl(e,t.alternate,t),t=t.sibling}function vl(e){for(e=e.child;e!==null;){var t=e;switch(t.tag){case 0:case 11:case 14:case 15:Rc(4,t,t.return),vl(t);break;case 1:Hc(t,t.return);var n=t.stateNode;typeof n.componentWillUnmount==`function`&&Bc(t,t.return,n),vl(t);break;case 27:pf(t.stateNode);case 26:case 5:Hc(t,t.return),vl(t);break;case 22:t.memoizedState===null&&vl(t);break;case 30:vl(t);break;default:vl(t)}e=e.sibling}}function yl(e,t,n){for(n&&=(t.subtreeFlags&8772)!=0,t=t.child;t!==null;){var r=t.alternate,i=e,a=t,o=a.flags;switch(a.tag){case 0:case 11:case 15:yl(i,a,n),Lc(4,a);break;case 1:if(yl(i,a,n),r=a,i=r.stateNode,typeof i.componentDidMount==`function`)try{i.componentDidMount()}catch(e){Z(r,r.return,e)}if(r=a,i=r.updateQueue,i!==null){var s=r.stateNode;try{var c=i.shared.hiddenCallbacks;if(c!==null)for(i.shared.hiddenCallbacks=null,i=0;i<c.length;i++)qa(c[i],s)}catch(e){Z(r,r.return,e)}}n&&o&64&&zc(a),Vc(a,a.return);break;case 27:Yc(a);case 26:case 5:yl(i,a,n),n&&r===null&&o&4&&Uc(a),Vc(a,a.return);break;case 12:yl(i,a,n);break;case 31:yl(i,a,n),n&&o&4&&cl(i,a);break;case 13:yl(i,a,n),n&&o&4&&ll(i,a);break;case 22:a.memoizedState===null&&yl(i,a,n),Vc(a,a.return);break;case 30:break;default:yl(i,a,n)}t=t.sibling}}function bl(e,t){var n=null;e!==null&&e.memoizedState!==null&&e.memoizedState.cachePool!==null&&(n=e.memoizedState.cachePool.pool),e=null,t.memoizedState!==null&&t.memoizedState.cachePool!==null&&(e=t.memoizedState.cachePool.pool),e!==n&&(e!=null&&e.refCount++,n!=null&&oa(n))}function xl(e,t){e=null,t.alternate!==null&&(e=t.alternate.memoizedState.cache),t=t.memoizedState.cache,t!==e&&(t.refCount++,e!=null&&oa(e))}function Sl(e,t,n,r){if(t.subtreeFlags&10256)for(t=t.child;t!==null;)Cl(e,t,n,r),t=t.sibling}function Cl(e,t,n,r){var i=t.flags;switch(t.tag){case 0:case 11:case 15:Sl(e,t,n,r),i&2048&&Lc(9,t);break;case 1:Sl(e,t,n,r);break;case 3:Sl(e,t,n,r),i&2048&&(e=null,t.alternate!==null&&(e=t.alternate.memoizedState.cache),t=t.memoizedState.cache,t!==e&&(t.refCount++,e!=null&&oa(e)));break;case 12:if(i&2048){Sl(e,t,n,r),e=t.stateNode;try{var a=t.memoizedProps,o=a.id,s=a.onPostCommit;typeof s==`function`&&s(o,t.alternate===null?`mount`:`update`,e.passiveEffectDuration,-0)}catch(e){Z(t,t.return,e)}}else Sl(e,t,n,r);break;case 31:Sl(e,t,n,r);break;case 13:Sl(e,t,n,r);break;case 23:break;case 22:a=t.stateNode,o=t.alternate,t.memoizedState===null?a._visibility&2?Sl(e,t,n,r):(a._visibility|=2,wl(e,t,n,r,(t.subtreeFlags&10256)!=0||!1)):a._visibility&2?Sl(e,t,n,r):Tl(e,t),i&2048&&bl(o,t);break;case 24:Sl(e,t,n,r),i&2048&&xl(t.alternate,t);break;default:Sl(e,t,n,r)}}function wl(e,t,n,r,i){for(i&&=(t.subtreeFlags&10256)!=0||!1,t=t.child;t!==null;){var a=e,o=t,s=n,c=r,l=o.flags;switch(o.tag){case 0:case 11:case 15:wl(a,o,s,c,i),Lc(8,o);break;case 23:break;case 22:var u=o.stateNode;o.memoizedState===null?(u._visibility|=2,wl(a,o,s,c,i)):u._visibility&2?wl(a,o,s,c,i):Tl(a,o),i&&l&2048&&bl(o.alternate,o);break;case 24:wl(a,o,s,c,i),i&&l&2048&&xl(o.alternate,o);break;default:wl(a,o,s,c,i)}t=t.sibling}}function Tl(e,t){if(t.subtreeFlags&10256)for(t=t.child;t!==null;){var n=e,r=t,i=r.flags;switch(r.tag){case 22:Tl(n,r),i&2048&&bl(r.alternate,r);break;case 24:Tl(n,r),i&2048&&xl(r.alternate,r);break;default:Tl(n,r)}t=t.sibling}}var El=8192;function Dl(e,t,n){if(e.subtreeFlags&El)for(e=e.child;e!==null;)Ol(e,t,n),e=e.sibling}function Ol(e,t,n){switch(e.tag){case 26:Dl(e,t,n),e.flags&El&&e.memoizedState!==null&&Gf(n,pl,e.memoizedState,e.memoizedProps);break;case 5:Dl(e,t,n);break;case 3:case 4:var r=pl;pl=gf(e.stateNode.containerInfo),Dl(e,t,n),pl=r;break;case 22:e.memoizedState===null&&(r=e.alternate,r!==null&&r.memoizedState!==null?(r=El,El=16777216,Dl(e,t,n),El=r):Dl(e,t,n));break;default:Dl(e,t,n)}}function kl(e){var t=e.alternate;if(t!==null&&(e=t.child,e!==null)){t.child=null;do t=e.sibling,e.sibling=null,e=t;while(e!==null)}}function Al(e){var t=e.deletions;if(e.flags&16){if(t!==null)for(var n=0;n<t.length;n++){var r=t[n];el=r,Nl(r,e)}kl(e)}if(e.subtreeFlags&10256)for(e=e.child;e!==null;)jl(e),e=e.sibling}function jl(e){switch(e.tag){case 0:case 11:case 15:Al(e),e.flags&2048&&Rc(9,e,e.return);break;case 3:Al(e);break;case 12:Al(e);break;case 22:var t=e.stateNode;e.memoizedState!==null&&t._visibility&2&&(e.return===null||e.return.tag!==13)?(t._visibility&=-3,Ml(e)):Al(e);break;default:Al(e)}}function Ml(e){var t=e.deletions;if(e.flags&16){if(t!==null)for(var n=0;n<t.length;n++){var r=t[n];el=r,Nl(r,e)}kl(e)}for(e=e.child;e!==null;){switch(t=e,t.tag){case 0:case 11:case 15:Rc(8,t,t.return),Ml(t);break;case 22:n=t.stateNode,n._visibility&2&&(n._visibility&=-3,Ml(t));break;default:Ml(t)}e=e.sibling}}function Nl(e,t){for(;el!==null;){var n=el;switch(n.tag){case 0:case 11:case 15:Rc(8,n,t);break;case 23:case 22:if(n.memoizedState!==null&&n.memoizedState.cachePool!==null){var r=n.memoizedState.cachePool.pool;r!=null&&r.refCount++}break;case 24:oa(n.memoizedState.cache)}if(r=n.child,r!==null)r.return=n,el=r;else a:for(n=e;el!==null;){r=el;var i=r.sibling,a=r.return;if(rl(r),r===n){el=null;break a}if(i!==null){i.return=a,el=i;break a}el=a}}}var Pl={getCacheForType:function(e){var t=Qi(ia),n=t.data.get(e);return n===void 0&&(n=e(),t.data.set(e,n)),n},cacheSignal:function(){return Qi(ia).controller.signal}},Fl=typeof WeakMap==`function`?WeakMap:Map,Il=0,Ll=null,Y=null,X=0,Rl=0,zl=null,Bl=!1,Vl=!1,Hl=!1,Ul=0,Wl=0,Gl=0,Kl=0,ql=0,Jl=0,Yl=0,Xl=null,Zl=null,Ql=!1,$l=0,eu=0,tu=1/0,nu=null,ru=null,iu=0,au=null,ou=null,su=0,cu=0,lu=null,uu=null,du=0,fu=null;function pu(){return Il&2&&X!==0?X&-X:M.T===null?nt():ud()}function mu(){if(Jl===0)if(!(X&536870912)||H){var e=Ve;Ve<<=1,!(Ve&3932160)&&(Ve=262144),Jl=e}else Jl=536870912;return e=eo.current,e!==null&&(e.flags|=32),Jl}function hu(e,t,n){(e===Ll&&(Rl===2||Rl===9)||e.cancelPendingCommit!==null)&&(Su(e,0),yu(e,X,Jl,!1)),Ye(e,n),(!(Il&2)||e!==Ll)&&(e===Ll&&(!(Il&2)&&(Kl|=n),Wl===4&&yu(e,X,Jl,!1)),nd(e))}function gu(e,t,n){if(Il&6)throw Error(a(327));var r=!n&&(t&127)==0&&(t&e.expiredLanes)===0||Ge(e,t),i=r?Au(e,t):Ou(e,t,!0),o=r;do{if(i===0){Vl&&!r&&yu(e,t,0,!1);break}else{if(n=e.current.alternate,o&&!vu(n)){i=Ou(e,t,!1),o=!1;continue}if(i===2){if(o=t,e.errorRecoveryDisabledLanes&o)var s=0;else s=e.pendingLanes&-536870913,s=s===0?s&536870912?536870912:0:s;if(s!==0){t=s;a:{var c=e;i=Xl;var l=c.current.memoizedState.isDehydrated;if(l&&(Su(c,s).flags|=256),s=Ou(c,s,!1),s!==2){if(Hl&&!l){c.errorRecoveryDisabledLanes|=o,Kl|=o,i=4;break a}o=Zl,Zl=i,o!==null&&(Zl===null?Zl=o:Zl.push.apply(Zl,o))}i=s}if(o=!1,i!==2)continue}}if(i===1){Su(e,0),yu(e,t,0,!0);break}a:{switch(r=e,o=i,o){case 0:case 1:throw Error(a(345));case 4:if((t&4194048)!==t)break;case 6:yu(r,t,Jl,!Bl);break a;case 2:Zl=null;break;case 3:case 5:break;default:throw Error(a(329))}if((t&62914560)===t&&(i=$l+300-we(),10<i)){if(yu(r,t,Jl,!Bl),We(r,0,!0)!==0)break a;su=t,r.timeoutHandle=Kd(_u.bind(null,r,n,Zl,nu,Ql,t,Jl,Kl,Yl,Bl,o,`Throttled`,-0,0),i);break a}_u(r,n,Zl,nu,Ql,t,Jl,Kl,Yl,Bl,o,null,-0,0)}}break}while(1);nd(e)}function _u(e,t,n,r,i,a,o,s,c,l,u,d,f,p){if(e.timeoutHandle=-1,d=t.subtreeFlags,d&8192||(d&16785408)==16785408){d={stylesheets:null,count:0,imgCount:0,imgBytes:0,suspenseyImages:[],waitingForImages:!0,waitingForViewTransition:!1,unsuspend:Qt},Ol(t,a,d);var m=(a&62914560)===a?$l-we():(a&4194048)===a?eu-we():0;if(m=qf(d,m),m!==null){su=a,e.cancelPendingCommit=m(Lu.bind(null,e,t,a,n,r,i,o,s,c,u,d,null,f,p)),yu(e,a,o,!l);return}}Lu(e,t,a,n,r,i,o,s,c)}function vu(e){for(var t=e;;){var n=t.tag;if((n===0||n===11||n===15)&&t.flags&16384&&(n=t.updateQueue,n!==null&&(n=n.stores,n!==null)))for(var r=0;r<n.length;r++){var i=n[r],a=i.getSnapshot;i=i.value;try{if(!xr(a(),i))return!1}catch{return!1}}if(n=t.child,t.subtreeFlags&16384&&n!==null)n.return=t,t=n;else{if(t===e)break;for(;t.sibling===null;){if(t.return===null||t.return===e)return!0;t=t.return}t.sibling.return=t.return,t=t.sibling}}return!0}function yu(e,t,n,r){t&=~ql,t&=~Kl,e.suspendedLanes|=t,e.pingedLanes&=~t,r&&(e.warmLanes|=t),r=e.expirationTimes;for(var i=t;0<i;){var a=31-Ie(i),o=1<<a;r[a]=-1,i&=~o}n!==0&&Ze(e,n,t)}function bu(){return Il&6?!0:(rd(0,!1),!1)}function xu(){if(Y!==null){if(Rl===0)var e=Y.return;else e=Y,Wi=Ui=null,Oo(e),ka=null,Aa=0,e=Y;for(;e!==null;)Ic(e.alternate,e),e=e.return;Y=null}}function Su(e,t){var n=e.timeoutHandle;n!==-1&&(e.timeoutHandle=-1,qd(n)),n=e.cancelPendingCommit,n!==null&&(e.cancelPendingCommit=null,n()),su=0,xu(),Ll=e,Y=n=si(e.current,null),X=t,Rl=0,zl=null,Bl=!1,Vl=Ge(e,t),Hl=!1,Yl=Jl=ql=Kl=Gl=Wl=0,Zl=Xl=null,Ql=!1,t&8&&(t|=t&32);var r=e.entangledLanes;if(r!==0)for(e=e.entanglements,r&=t;0<r;){var i=31-Ie(r),a=1<<i;t|=e[i],r&=~a}return Ul=t,Zr(),n}function Cu(e,t){U=null,M.H=Ps,t===ya||t===xa?(t=Da(),Rl=3):t===ba?(t=Da(),Rl=4):Rl=t===Zs?8:typeof t==`object`&&t&&typeof t.then==`function`?6:1,zl=t,Y===null&&(Wl=1,Ks(e,hi(t,e.current)))}function wu(){var e=eo.current;return e===null?!0:(X&4194048)===X?to===null:(X&62914560)===X||X&536870912?e===to:!1}function Tu(){var e=M.H;return M.H=Ps,e===null?Ps:e}function Eu(){var e=M.A;return M.A=Pl,e}function Du(){Wl=4,Bl||(X&4194048)!==X&&eo.current!==null||(Vl=!0),!(Gl&134217727)&&!(Kl&134217727)||Ll===null||yu(Ll,X,Jl,!1)}function Ou(e,t,n){var r=Il;Il|=2;var i=Tu(),a=Eu();(Ll!==e||X!==t)&&(nu=null,Su(e,t)),t=!1;var o=Wl;a:do try{if(Rl!==0&&Y!==null){var s=Y,c=zl;switch(Rl){case 8:xu(),o=6;break a;case 3:case 2:case 9:case 6:eo.current===null&&(t=!0);var l=Rl;if(Rl=0,zl=null,Pu(e,s,c,l),n&&Vl){o=0;break a}break;default:l=Rl,Rl=0,zl=null,Pu(e,s,c,l)}}ku(),o=Wl;break}catch(t){Cu(e,t)}while(1);return t&&e.shellSuspendCounter++,Wi=Ui=null,Il=r,M.H=i,M.A=a,Y===null&&(Ll=null,X=0,Zr()),o}function ku(){for(;Y!==null;)Mu(Y)}function Au(e,t){var n=Il;Il|=2;var r=Tu(),i=Eu();Ll!==e||X!==t?(nu=null,tu=we()+500,Su(e,t)):Vl=Ge(e,t);a:do try{if(Rl!==0&&Y!==null){t=Y;var o=zl;b:switch(Rl){case 1:Rl=0,zl=null,Pu(e,t,o,1);break;case 2:case 9:if(Ca(o)){Rl=0,zl=null,Nu(t);break}t=function(){Rl!==2&&Rl!==9||Ll!==e||(Rl=7),nd(e)},o.then(t,t);break a;case 3:Rl=7;break a;case 4:Rl=5;break a;case 7:Ca(o)?(Rl=0,zl=null,Nu(t)):(Rl=0,zl=null,Pu(e,t,o,7));break;case 5:var s=null;switch(Y.tag){case 26:s=Y.memoizedState;case 5:case 27:var c=Y;if(s?Wf(s):c.stateNode.complete){Rl=0,zl=null;var l=c.sibling;if(l!==null)Y=l;else{var u=c.return;u===null?Y=null:(Y=u,Fu(u))}break b}}Rl=0,zl=null,Pu(e,t,o,5);break;case 6:Rl=0,zl=null,Pu(e,t,o,6);break;case 8:xu(),Wl=6;break a;default:throw Error(a(462))}}ju();break}catch(t){Cu(e,t)}while(1);return Wi=Ui=null,M.H=r,M.A=i,Il=n,Y===null?(Ll=null,X=0,Zr(),Wl):0}function ju(){for(;Y!==null&&!Se();)Mu(Y)}function Mu(e){var t=Dc(e.alternate,e,Ul);e.memoizedProps=e.pendingProps,t===null?Fu(e):Y=t}function Nu(e){var t=e,n=t.alternate;switch(t.tag){case 15:case 0:t=dc(n,t,t.pendingProps,t.type,void 0,X);break;case 11:t=dc(n,t,t.pendingProps,t.type.render,t.ref,X);break;case 5:Oo(t);default:Ic(n,t),t=Y=ci(t,Ul),t=Dc(n,t,Ul)}e.memoizedProps=e.pendingProps,t===null?Fu(e):Y=t}function Pu(e,t,n,r){Wi=Ui=null,Oo(t),ka=null,Aa=0;var i=t.return;try{if(J(e,i,t,n,X)){Wl=1,Ks(e,hi(n,e.current)),Y=null;return}}catch(t){if(i!==null)throw Y=i,t;Wl=1,Ks(e,hi(n,e.current)),Y=null;return}t.flags&32768?(H||r===1?e=!0:Vl||X&536870912?e=!1:(Bl=e=!0,(r===2||r===9||r===3||r===6)&&(r=eo.current,r!==null&&r.tag===13&&(r.flags|=16384))),Iu(t,e)):Fu(t)}function Fu(e){var t=e;do{if(t.flags&32768){Iu(t,Bl);return}e=t.return;var n=Pc(t.alternate,t,Ul);if(n!==null){Y=n;return}if(t=t.sibling,t!==null){Y=t;return}Y=t=e}while(t!==null);Wl===0&&(Wl=5)}function Iu(e,t){do{var n=Fc(e.alternate,e);if(n!==null){n.flags&=32767,Y=n;return}if(n=e.return,n!==null&&(n.flags|=32768,n.subtreeFlags=0,n.deletions=null),!t&&(e=e.sibling,e!==null)){Y=e;return}Y=e=n}while(e!==null);Wl=6,Y=null}function Lu(e,t,n,r,i,o,s,c,l){e.cancelPendingCommit=null;do Hu();while(iu!==0);if(Il&6)throw Error(a(327));if(t!==null){if(t===e.current)throw Error(a(177));if(o=t.lanes|t.childLanes,o|=Xr,Xe(e,n,o,s,c,l),e===Ll&&(Y=Ll=null,X=0),ou=t,au=e,su=n,cu=o,lu=i,uu=r,t.subtreeFlags&10256||t.flags&10256?(e.callbackNode=null,e.callbackPriority=0,Q(Oe,function(){return Uu(),null})):(e.callbackNode=null,e.callbackPriority=0),r=(t.flags&13878)!=0,t.subtreeFlags&13878||r){r=M.T,M.T=null,i=N.p,N.p=2,s=Il,Il|=4;try{tl(e,t,n)}finally{Il=s,N.p=i,M.T=r}}iu=1,Ru(),zu(),Bu()}}function Ru(){if(iu===1){iu=0;var e=au,t=ou,n=(t.flags&13878)!=0;if(t.subtreeFlags&13878||n){n=M.T,M.T=null;var r=N.p;N.p=2;var i=Il;Il|=4;try{ml(t,e);var a=zd,o=Er(e.containerInfo),s=a.focusedElem,c=a.selectionRange;if(o!==s&&s&&s.ownerDocument&&Tr(s.ownerDocument.documentElement,s)){if(c!==null&&Dr(s)){var l=c.start,u=c.end;if(u===void 0&&(u=l),`selectionStart`in s)s.selectionStart=l,s.selectionEnd=Math.min(u,s.value.length);else{var d=s.ownerDocument||document,f=d&&d.defaultView||window;if(f.getSelection){var p=f.getSelection(),m=s.textContent.length,h=Math.min(c.start,m),g=c.end===void 0?h:Math.min(c.end,m);!p.extend&&h>g&&(o=g,g=h,h=o);var _=wr(s,h),v=wr(s,g);if(_&&v&&(p.rangeCount!==1||p.anchorNode!==_.node||p.anchorOffset!==_.offset||p.focusNode!==v.node||p.focusOffset!==v.offset)){var y=d.createRange();y.setStart(_.node,_.offset),p.removeAllRanges(),h>g?(p.addRange(y),p.extend(v.node,v.offset)):(y.setEnd(v.node,v.offset),p.addRange(y))}}}}for(d=[],p=s;p=p.parentNode;)p.nodeType===1&&d.push({element:p,left:p.scrollLeft,top:p.scrollTop});for(typeof s.focus==`function`&&s.focus(),s=0;s<d.length;s++){var b=d[s];b.element.scrollLeft=b.left,b.element.scrollTop=b.top}}sp=!!Rd,zd=Rd=null}finally{Il=i,N.p=r,M.T=n}}e.current=t,iu=2}}function zu(){if(iu===2){iu=0;var e=au,t=ou,n=(t.flags&8772)!=0;if(t.subtreeFlags&8772||n){n=M.T,M.T=null;var r=N.p;N.p=2;var i=Il;Il|=4;try{nl(e,t.alternate,t)}finally{Il=i,N.p=r,M.T=n}}iu=3}}function Bu(){if(iu===4||iu===3){iu=0,Ce();var e=au,t=ou,n=su,r=uu;t.subtreeFlags&10256||t.flags&10256?iu=5:(iu=0,ou=au=null,Vu(e,e.pendingLanes));var i=e.pendingLanes;if(i===0&&(ru=null),tt(n),t=t.stateNode,Pe&&typeof Pe.onCommitFiberRoot==`function`)try{Pe.onCommitFiberRoot(Ne,t,void 0,(t.current.flags&128)==128)}catch{}if(r!==null){t=M.T,i=N.p,N.p=2,M.T=null;try{for(var a=e.onRecoverableError,o=0;o<r.length;o++){var s=r[o];a(s.value,{componentStack:s.stack})}}finally{M.T=t,N.p=i}}su&3&&Hu(),nd(e),i=e.pendingLanes,n&261930&&i&42?e===fu?du++:(du=0,fu=e):du=0,rd(0,!1)}}function Vu(e,t){(e.pooledCacheLanes&=t)===0&&(t=e.pooledCache,t!=null&&(e.pooledCache=null,oa(t)))}function Hu(){return Ru(),zu(),Bu(),Uu()}function Uu(){if(iu!==5)return!1;var e=au,t=cu;cu=0;var n=tt(su),r=M.T,i=N.p;try{N.p=32>n?32:n,M.T=null,n=lu,lu=null;var o=au,s=su;if(iu=0,ou=au=null,su=0,Il&6)throw Error(a(331));var c=Il;if(Il|=4,jl(o.current),Cl(o,o.current,s,n),Il=c,rd(0,!1),Pe&&typeof Pe.onPostCommitFiberRoot==`function`)try{Pe.onPostCommitFiberRoot(Ne,o)}catch{}return!0}finally{N.p=i,M.T=r,Vu(e,t)}}function Wu(e,t,n){t=hi(n,t),t=Js(e.stateNode,t,2),e=Va(e,t,2),e!==null&&(Ye(e,2),nd(e))}function Z(e,t,n){if(e.tag===3)Wu(e,e,n);else for(;t!==null;){if(t.tag===3){Wu(t,e,n);break}else if(t.tag===1){var r=t.stateNode;if(typeof t.type.getDerivedStateFromError==`function`||typeof r.componentDidCatch==`function`&&(ru===null||!ru.has(r))){e=hi(n,e),n=Ys(2),r=Va(t,n,2),r!==null&&(Xs(n,r,t,e),Ye(r,2),nd(r));break}}t=t.return}}function Gu(e,t,n){var r=e.pingCache;if(r===null){r=e.pingCache=new Fl;var i=new Set;r.set(t,i)}else i=r.get(t),i===void 0&&(i=new Set,r.set(t,i));i.has(n)||(Hl=!0,i.add(n),e=Ku.bind(null,e,t,n),t.then(e,e))}function Ku(e,t,n){var r=e.pingCache;r!==null&&r.delete(t),e.pingedLanes|=e.suspendedLanes&n,e.warmLanes&=~n,Ll===e&&(X&n)===n&&(Wl===4||Wl===3&&(X&62914560)===X&&300>we()-$l?!(Il&2)&&Su(e,0):ql|=n,Yl===X&&(Yl=0)),nd(e)}function qu(e,t){t===0&&(t=qe()),e=ei(e,t),e!==null&&(Ye(e,t),nd(e))}function Ju(e){var t=e.memoizedState,n=0;t!==null&&(n=t.retryLane),qu(e,n)}function Yu(e,t){var n=0;switch(e.tag){case 31:case 13:var r=e.stateNode,i=e.memoizedState;i!==null&&(n=i.retryLane);break;case 19:r=e.stateNode;break;case 22:r=e.stateNode._retryCache;break;default:throw Error(a(314))}r!==null&&r.delete(t),qu(e,n)}function Q(e,t){return be(e,t)}var Xu=null,Zu=null,Qu=!1,$u=!1,ed=!1,td=0;function nd(e){e!==Zu&&e.next===null&&(Zu===null?Xu=Zu=e:Zu=Zu.next=e),$u=!0,Qu||(Qu=!0,ld())}function rd(e,t){if(!ed&&$u){ed=!0;do for(var n=!1,r=Xu;r!==null;){if(!t)if(e!==0){var i=r.pendingLanes;if(i===0)var a=0;else{var o=r.suspendedLanes,s=r.pingedLanes;a=(1<<31-Ie(42|e)+1)-1,a&=i&~(o&~s),a=a&201326741?a&201326741|1:a?a|2:0}a!==0&&(n=!0,cd(r,a))}else a=X,a=We(r,r===Ll?a:0,r.cancelPendingCommit!==null||r.timeoutHandle!==-1),!(a&3)||Ge(r,a)||(n=!0,cd(r,a));r=r.next}while(n);ed=!1}}function id(){ad()}function ad(){$u=Qu=!1;var e=0;td!==0&&Gd()&&(e=td);for(var t=we(),n=null,r=Xu;r!==null;){var i=r.next,a=od(r,t);a===0?(r.next=null,n===null?Xu=i:n.next=i,i===null&&(Zu=n)):(n=r,(e!==0||a&3)&&($u=!0)),r=i}iu!==0&&iu!==5||rd(e,!1),td!==0&&(td=0)}function od(e,t){for(var n=e.suspendedLanes,r=e.pingedLanes,i=e.expirationTimes,a=e.pendingLanes&-62914561;0<a;){var o=31-Ie(a),s=1<<o,c=i[o];c===-1?((s&n)===0||(s&r)!==0)&&(i[o]=Ke(s,t)):c<=t&&(e.expiredLanes|=s),a&=~s}if(t=Ll,n=X,n=We(e,e===t?n:0,e.cancelPendingCommit!==null||e.timeoutHandle!==-1),r=e.callbackNode,n===0||e===t&&(Rl===2||Rl===9)||e.cancelPendingCommit!==null)return r!==null&&r!==null&&xe(r),e.callbackNode=null,e.callbackPriority=0;if(!(n&3)||Ge(e,n)){if(t=n&-n,t===e.callbackPriority)return t;switch(r!==null&&xe(r),tt(n)){case 2:case 8:n=De;break;case 32:n=Oe;break;case 268435456:n=Ae;break;default:n=Oe}return r=sd.bind(null,e),n=be(n,r),e.callbackPriority=t,e.callbackNode=n,t}return r!==null&&r!==null&&xe(r),e.callbackPriority=2,e.callbackNode=null,2}function sd(e,t){if(iu!==0&&iu!==5)return e.callbackNode=null,e.callbackPriority=0,null;var n=e.callbackNode;if(Hu()&&e.callbackNode!==n)return null;var r=X;return r=We(e,e===Ll?r:0,e.cancelPendingCommit!==null||e.timeoutHandle!==-1),r===0?null:(gu(e,r,t),od(e,we()),e.callbackNode!=null&&e.callbackNode===n?sd.bind(null,e):null)}function cd(e,t){if(Hu())return null;gu(e,t,!0)}function ld(){Yd(function(){Il&6?be(Ee,id):ad()})}function ud(){if(td===0){var e=la;e===0&&(e=Be,Be<<=1,!(Be&261888)&&(Be=256)),td=e}return td}function dd(e){return e==null||typeof e==`symbol`||typeof e==`boolean`?null:typeof e==`function`?e:Zt(``+e)}function fd(e,t){var n=t.ownerDocument.createElement(`input`);return n.name=t.name,n.value=t.value,e.id&&n.setAttribute(`form`,e.id),t.parentNode.insertBefore(n,t),e=new FormData(e),n.parentNode.removeChild(n),e}function pd(e,t,n,r,i){if(t===`submit`&&n&&n.stateNode===i){var a=dd((i[ot]||null).action),o=r.submitter;o&&(t=(t=o[ot]||null)?dd(t.formAction):o.getAttribute(`formAction`),t!==null&&(a=t,o=null));var s=new bn(`action`,`action`,null,r,i);e.push({event:s,listeners:[{instance:null,listener:function(){if(r.defaultPrevented){if(td!==0){var e=o?fd(i,o):new FormData(i);Ss(n,{pending:!0,data:e,method:i.method,action:a},null,e)}}else typeof a==`function`&&(s.preventDefault(),e=o?fd(i,o):new FormData(i),Ss(n,{pending:!0,data:e,method:i.method,action:a},a,e))},currentTarget:i}]})}}for(var md=0;md<Kr.length;md++){var hd=Kr[md];V(hd.toLowerCase(),`on`+(hd[0].toUpperCase()+hd.slice(1)))}V(zr,`onAnimationEnd`),V(Br,`onAnimationIteration`),V(Vr,`onAnimationStart`),V(`dblclick`,`onDoubleClick`),V(`focusin`,`onFocus`),V(`focusout`,`onBlur`),V(Hr,`onTransitionRun`),V(B,`onTransitionStart`),V(Ur,`onTransitionCancel`),V(Wr,`onTransitionEnd`),St(`onMouseEnter`,[`mouseout`,`mouseover`]),St(`onMouseLeave`,[`mouseout`,`mouseover`]),St(`onPointerEnter`,[`pointerout`,`pointerover`]),St(`onPointerLeave`,[`pointerout`,`pointerover`]),xt(`onChange`,`change click focusin focusout input keydown keyup selectionchange`.split(` `)),xt(`onSelect`,`focusout contextmenu dragend focusin keydown keyup mousedown mouseup selectionchange`.split(` `)),xt(`onBeforeInput`,[`compositionend`,`keypress`,`textInput`,`paste`]),xt(`onCompositionEnd`,`compositionend focusout keydown keypress keyup mousedown`.split(` `)),xt(`onCompositionStart`,`compositionstart focusout keydown keypress keyup mousedown`.split(` `)),xt(`onCompositionUpdate`,`compositionupdate focusout keydown keypress keyup mousedown`.split(` `));var gd=`abort canplay canplaythrough durationchange emptied encrypted ended error loadeddata loadedmetadata loadstart pause play playing progress ratechange resize seeked seeking stalled suspend timeupdate volumechange waiting`.split(` `),_d=new Set(`beforetoggle cancel close invalid load scroll scrollend toggle`.split(` `).concat(gd));function vd(e,t){t=(t&4)!=0;for(var n=0;n<e.length;n++){var r=e[n],i=r.event;r=r.listeners;a:{var a=void 0;if(t)for(var o=r.length-1;0<=o;o--){var s=r[o],c=s.instance,l=s.currentTarget;if(s=s.listener,c!==a&&i.isPropagationStopped())break a;a=s,i.currentTarget=l;try{a(i)}catch(e){qr(e)}i.currentTarget=null,a=c}else for(o=0;o<r.length;o++){if(s=r[o],c=s.instance,l=s.currentTarget,s=s.listener,c!==a&&i.isPropagationStopped())break a;a=s,i.currentTarget=l;try{a(i)}catch(e){qr(e)}i.currentTarget=null,a=c}}}}function $(e,t){var n=t[ct];n===void 0&&(n=t[ct]=new Set);var r=e+`__bubble`;n.has(r)||(Sd(t,e,2,!1),n.add(r))}function yd(e,t,n){var r=0;t&&(r|=4),Sd(n,e,r,t)}var bd=`_reactListening`+Math.random().toString(36).slice(2);function xd(e){if(!e[bd]){e[bd]=!0,yt.forEach(function(t){t!==`selectionchange`&&(_d.has(t)||yd(t,!1,e),yd(t,!0,e))});var t=e.nodeType===9?e:e.ownerDocument;t===null||t[bd]||(t[bd]=!0,yd(`selectionchange`,!1,t))}}function Sd(e,t,n,r){switch(mp(t)){case 2:var i=cp;break;case 8:i=lp;break;default:i=up}n=i.bind(null,t,n,e),i=void 0,!ln||t!==`touchstart`&&t!==`touchmove`&&t!==`wheel`||(i=!0),r?i===void 0?e.addEventListener(t,n,!0):e.addEventListener(t,n,{capture:!0,passive:i}):i===void 0?e.addEventListener(t,n,!1):e.addEventListener(t,n,{passive:i})}function Cd(e,t,n,r,i){var a=r;if(!(t&1)&&!(t&2)&&r!==null)a:for(;;){if(r===null)return;var o=r.tag;if(o===3||o===4){var c=r.stateNode.containerInfo;if(c===i)break;if(o===4)for(o=r.return;o!==null;){var l=o.tag;if((l===3||l===4)&&o.stateNode.containerInfo===i)return;o=o.return}for(;c!==null;){if(o=mt(c),o===null)return;if(l=o.tag,l===5||l===6||l===26||l===27){r=a=o;continue a}c=c.parentNode}}r=r.return}on(function(){var r=a,i=en(n),o=[];a:{var c=Gr.get(e);if(c!==void 0){var l=bn,u=e;switch(e){case`keypress`:if(hn(n)===0)break a;case`keydown`:case`keyup`:l=Rn;break;case`focusin`:u=`focus`,l=kn;break;case`focusout`:u=`blur`,l=kn;break;case`beforeblur`:case`afterblur`:l=kn;break;case`click`:if(n.button===2)break a;case`auxclick`:case`dblclick`:case`mousedown`:case`mousemove`:case`mouseup`:case`mouseout`:case`mouseover`:case`contextmenu`:l=Dn;break;case`drag`:case`dragend`:case`dragenter`:case`dragexit`:case`dragleave`:case`dragover`:case`dragstart`:case`drop`:l=On;break;case`touchcancel`:case`touchend`:case`touchmove`:case`touchstart`:l=Bn;break;case zr:case Br:case Vr:l=An;break;case Wr:l=Vn;break;case`scroll`:case`scrollend`:l=Sn;break;case`wheel`:l=Hn;break;case`copy`:case`cut`:case`paste`:l=jn;break;case`gotpointercapture`:case`lostpointercapture`:case`pointercancel`:case`pointerdown`:case`pointermove`:case`pointerout`:case`pointerover`:case`pointerup`:l=zn;break;case`toggle`:case`beforetoggle`:l=Un}var d=(t&4)!=0,f=!d&&(e===`scroll`||e===`scrollend`),p=d?c===null?null:c+`Capture`:c;d=[];for(var m=r,h;m!==null;){var g=m;if(h=g.stateNode,g=g.tag,g!==5&&g!==26&&g!==27||h===null||p===null||(g=sn(m,p),g!=null&&d.push(wd(m,g,h))),f)break;m=m.return}0<d.length&&(c=new l(c,u,null,n,i),o.push({event:c,listeners:d}))}}if(!(t&7)){a:{if(c=e===`mouseover`||e===`pointerover`,l=e===`mouseout`||e===`pointerout`,c&&n!==$t&&(u=n.relatedTarget||n.fromElement)&&(mt(u)||u[st]))break a;if((l||c)&&(c=i.window===i?i:(c=i.ownerDocument)?c.defaultView||c.parentWindow:window,l?(u=n.relatedTarget||n.toElement,l=r,u=u?mt(u):null,u!==null&&(f=s(u),d=u.tag,u!==f||d!==5&&d!==27&&d!==6)&&(u=null)):(l=null,u=r),l!==u)){if(d=Dn,g=`onMouseLeave`,p=`onMouseEnter`,m=`mouse`,(e===`pointerout`||e===`pointerover`)&&(d=zn,g=`onPointerLeave`,p=`onPointerEnter`,m=`pointer`),f=l==null?c:gt(l),h=u==null?c:gt(u),c=new d(g,m+`leave`,l,n,i),c.target=f,c.relatedTarget=h,g=null,mt(i)===r&&(d=new d(p,m+`enter`,u,n,i),d.target=h,d.relatedTarget=f,g=d),f=g,l&&u)b:{for(d=Ed,p=l,m=u,h=0,g=p;g;g=d(g))h++;g=0;for(var _=m;_;_=d(_))g++;for(;0<h-g;)p=d(p),h--;for(;0<g-h;)m=d(m),g--;for(;h--;){if(p===m||m!==null&&p===m.alternate){d=p;break b}p=d(p),m=d(m)}d=null}else d=null;l!==null&&Dd(o,c,l,d,!1),u!==null&&f!==null&&Dd(o,f,u,d,!0)}}a:{if(c=r?gt(r):window,l=c.nodeName&&c.nodeName.toLowerCase(),l===`select`||l===`input`&&c.type===`file`)var v=lr;else if(rr(c))if(ur)v=yr;else{v=_r;var y=gr}else l=c.nodeName,!l||l.toLowerCase()!==`input`||c.type!==`checkbox`&&c.type!==`radio`?r&&Jt(r.elementType)&&(v=lr):v=vr;if(v&&=v(e,r)){ir(o,v,n,i);break a}y&&y(e,c,r),e===`focusout`&&r&&c.type===`number`&&r.memoizedProps.value!=null&&Bt(c,`number`,c.value)}switch(y=r?gt(r):window,e){case`focusin`:(rr(y)||y.contentEditable===`true`)&&(kr=y,Ar=r,jr=null);break;case`focusout`:jr=Ar=kr=null;break;case`mousedown`:Mr=!0;break;case`contextmenu`:case`mouseup`:case`dragend`:Mr=!1,Nr(o,n,i);break;case`selectionchange`:if(Or)break;case`keydown`:case`keyup`:Nr(o,n,i)}var b;if(Gn)b:{switch(e){case`compositionstart`:var x=`onCompositionStart`;break b;case`compositionend`:x=`onCompositionEnd`;break b;case`compositionupdate`:x=`onCompositionUpdate`;break b}x=void 0}else $n?Zn(e,n)&&(x=`onCompositionEnd`):e===`keydown`&&n.keyCode===229&&(x=`onCompositionStart`);x&&(Jn&&n.locale!==`ko`&&($n||x!==`onCompositionStart`?x===`onCompositionEnd`&&$n&&(b=mn()):(dn=i,fn=`value`in dn?dn.value:dn.textContent,$n=!0)),y=Td(r,x),0<y.length&&(x=new Mn(x,e,null,n,i),o.push({event:x,listeners:y}),b?x.data=b:(b=Qn(n),b!==null&&(x.data=b)))),(b=qn?er(e,n):tr(e,n))&&(x=Td(r,`onBeforeInput`),0<x.length&&(y=new Mn(`onBeforeInput`,`beforeinput`,null,n,i),o.push({event:y,listeners:x}),y.data=b)),pd(o,e,r,n,i)}vd(o,t)})}function wd(e,t,n){return{instance:e,listener:t,currentTarget:n}}function Td(e,t){for(var n=t+`Capture`,r=[];e!==null;){var i=e,a=i.stateNode;if(i=i.tag,i!==5&&i!==26&&i!==27||a===null||(i=sn(e,n),i!=null&&r.unshift(wd(e,i,a)),i=sn(e,t),i!=null&&r.push(wd(e,i,a))),e.tag===3)return r;e=e.return}return[]}function Ed(e){if(e===null)return null;do e=e.return;while(e&&e.tag!==5&&e.tag!==27);return e||null}function Dd(e,t,n,r,i){for(var a=t._reactName,o=[];n!==null&&n!==r;){var s=n,c=s.alternate,l=s.stateNode;if(s=s.tag,c!==null&&c===r)break;s!==5&&s!==26&&s!==27||l===null||(c=l,i?(l=sn(n,a),l!=null&&o.unshift(wd(n,l,c))):i||(l=sn(n,a),l!=null&&o.push(wd(n,l,c)))),n=n.return}o.length!==0&&e.push({event:t,listeners:o})}var Od=/\r\n?/g,kd=/\u0000|\uFFFD/g;function Ad(e){return(typeof e==`string`?e:``+e).replace(Od,`
`).replace(kd,``)}function jd(e,t){return t=Ad(t),Ad(e)===t}function Md(e,t,n,r,i,o){switch(n){case`children`:typeof r==`string`?t===`body`||t===`textarea`&&r===``||Wt(e,r):(typeof r==`number`||typeof r==`bigint`)&&t!==`body`&&Wt(e,``+r);break;case`className`:Ot(e,`class`,r);break;case`tabIndex`:Ot(e,`tabindex`,r);break;case`dir`:case`role`:case`viewBox`:case`width`:case`height`:Ot(e,n,r);break;case`style`:qt(e,r,o);break;case`data`:if(t!==`object`){Ot(e,`data`,r);break}case`src`:case`href`:if(r===``&&(t!==`a`||n!==`href`)){e.removeAttribute(n);break}if(r==null||typeof r==`function`||typeof r==`symbol`||typeof r==`boolean`){e.removeAttribute(n);break}r=Zt(``+r),e.setAttribute(n,r);break;case`action`:case`formAction`:if(typeof r==`function`){e.setAttribute(n,`javascript:throw new Error('A React form was unexpectedly submitted. If you called form.submit() manually, consider using form.requestSubmit() instead. If you\\'re trying to use event.stopPropagation() in a submit event handler, consider also calling event.preventDefault().')`);break}else typeof o==`function`&&(n===`formAction`?(t!==`input`&&Md(e,t,`name`,i.name,i,null),Md(e,t,`formEncType`,i.formEncType,i,null),Md(e,t,`formMethod`,i.formMethod,i,null),Md(e,t,`formTarget`,i.formTarget,i,null)):(Md(e,t,`encType`,i.encType,i,null),Md(e,t,`method`,i.method,i,null),Md(e,t,`target`,i.target,i,null)));if(r==null||typeof r==`symbol`||typeof r==`boolean`){e.removeAttribute(n);break}r=Zt(``+r),e.setAttribute(n,r);break;case`onClick`:r!=null&&(e.onclick=Qt);break;case`onScroll`:r!=null&&$(`scroll`,e);break;case`onScrollEnd`:r!=null&&$(`scrollend`,e);break;case`dangerouslySetInnerHTML`:if(r!=null){if(typeof r!=`object`||!(`__html`in r))throw Error(a(61));if(n=r.__html,n!=null){if(i.children!=null)throw Error(a(60));e.innerHTML=n}}break;case`multiple`:e.multiple=r&&typeof r!=`function`&&typeof r!=`symbol`;break;case`muted`:e.muted=r&&typeof r!=`function`&&typeof r!=`symbol`;break;case`suppressContentEditableWarning`:case`suppressHydrationWarning`:case`defaultValue`:case`defaultChecked`:case`innerHTML`:case`ref`:break;case`autoFocus`:break;case`xlinkHref`:if(r==null||typeof r==`function`||typeof r==`boolean`||typeof r==`symbol`){e.removeAttribute(`xlink:href`);break}n=Zt(``+r),e.setAttributeNS(`http://www.w3.org/1999/xlink`,`xlink:href`,n);break;case`contentEditable`:case`spellCheck`:case`draggable`:case`value`:case`autoReverse`:case`externalResourcesRequired`:case`focusable`:case`preserveAlpha`:r!=null&&typeof r!=`function`&&typeof r!=`symbol`?e.setAttribute(n,``+r):e.removeAttribute(n);break;case`inert`:case`allowFullScreen`:case`async`:case`autoPlay`:case`controls`:case`default`:case`defer`:case`disabled`:case`disablePictureInPicture`:case`disableRemotePlayback`:case`formNoValidate`:case`hidden`:case`loop`:case`noModule`:case`noValidate`:case`open`:case`playsInline`:case`readOnly`:case`required`:case`reversed`:case`scoped`:case`seamless`:case`itemScope`:r&&typeof r!=`function`&&typeof r!=`symbol`?e.setAttribute(n,``):e.removeAttribute(n);break;case`capture`:case`download`:!0===r?e.setAttribute(n,``):!1!==r&&r!=null&&typeof r!=`function`&&typeof r!=`symbol`?e.setAttribute(n,r):e.removeAttribute(n);break;case`cols`:case`rows`:case`size`:case`span`:r!=null&&typeof r!=`function`&&typeof r!=`symbol`&&!isNaN(r)&&1<=r?e.setAttribute(n,r):e.removeAttribute(n);break;case`rowSpan`:case`start`:r==null||typeof r==`function`||typeof r==`symbol`||isNaN(r)?e.removeAttribute(n):e.setAttribute(n,r);break;case`popover`:$(`beforetoggle`,e),$(`toggle`,e),Dt(e,`popover`,r);break;case`xlinkActuate`:kt(e,`http://www.w3.org/1999/xlink`,`xlink:actuate`,r);break;case`xlinkArcrole`:kt(e,`http://www.w3.org/1999/xlink`,`xlink:arcrole`,r);break;case`xlinkRole`:kt(e,`http://www.w3.org/1999/xlink`,`xlink:role`,r);break;case`xlinkShow`:kt(e,`http://www.w3.org/1999/xlink`,`xlink:show`,r);break;case`xlinkTitle`:kt(e,`http://www.w3.org/1999/xlink`,`xlink:title`,r);break;case`xlinkType`:kt(e,`http://www.w3.org/1999/xlink`,`xlink:type`,r);break;case`xmlBase`:kt(e,`http://www.w3.org/XML/1998/namespace`,`xml:base`,r);break;case`xmlLang`:kt(e,`http://www.w3.org/XML/1998/namespace`,`xml:lang`,r);break;case`xmlSpace`:kt(e,`http://www.w3.org/XML/1998/namespace`,`xml:space`,r);break;case`is`:Dt(e,`is`,r);break;case`innerText`:case`textContent`:break;default:(!(2<n.length)||n[0]!==`o`&&n[0]!==`O`||n[1]!==`n`&&n[1]!==`N`)&&(n=Yt.get(n)||n,Dt(e,n,r))}}function Nd(e,t,n,r,i,o){switch(n){case`style`:qt(e,r,o);break;case`dangerouslySetInnerHTML`:if(r!=null){if(typeof r!=`object`||!(`__html`in r))throw Error(a(61));if(n=r.__html,n!=null){if(i.children!=null)throw Error(a(60));e.innerHTML=n}}break;case`children`:typeof r==`string`?Wt(e,r):(typeof r==`number`||typeof r==`bigint`)&&Wt(e,``+r);break;case`onScroll`:r!=null&&$(`scroll`,e);break;case`onScrollEnd`:r!=null&&$(`scrollend`,e);break;case`onClick`:r!=null&&(e.onclick=Qt);break;case`suppressContentEditableWarning`:case`suppressHydrationWarning`:case`innerHTML`:case`ref`:break;case`innerText`:case`textContent`:break;default:if(!bt.hasOwnProperty(n))a:{if(n[0]===`o`&&n[1]===`n`&&(i=n.endsWith(`Capture`),t=n.slice(2,i?n.length-7:void 0),o=e[ot]||null,o=o==null?null:o[n],typeof o==`function`&&e.removeEventListener(t,o,i),typeof r==`function`)){typeof o!=`function`&&o!==null&&(n in e?e[n]=null:e.hasAttribute(n)&&e.removeAttribute(n)),e.addEventListener(t,r,i);break a}n in e?e[n]=r:!0===r?e.setAttribute(n,``):Dt(e,n,r)}}}function Pd(e,t,n){switch(t){case`div`:case`span`:case`svg`:case`path`:case`a`:case`g`:case`p`:case`li`:break;case`img`:$(`error`,e),$(`load`,e);var r=!1,i=!1,o;for(o in n)if(n.hasOwnProperty(o)){var s=n[o];if(s!=null)switch(o){case`src`:r=!0;break;case`srcSet`:i=!0;break;case`children`:case`dangerouslySetInnerHTML`:throw Error(a(137,t));default:Md(e,t,o,s,n,null)}}i&&Md(e,t,`srcSet`,n.srcSet,n,null),r&&Md(e,t,`src`,n.src,n,null);return;case`input`:$(`invalid`,e);var c=o=s=i=null,l=null,u=null;for(r in n)if(n.hasOwnProperty(r)){var d=n[r];if(d!=null)switch(r){case`name`:i=d;break;case`type`:s=d;break;case`checked`:l=d;break;case`defaultChecked`:u=d;break;case`value`:o=d;break;case`defaultValue`:c=d;break;case`children`:case`dangerouslySetInnerHTML`:if(d!=null)throw Error(a(137,t));break;default:Md(e,t,r,d,n,null)}}zt(e,o,c,l,u,s,i,!1);return;case`select`:for(i in $(`invalid`,e),r=s=o=null,n)if(n.hasOwnProperty(i)&&(c=n[i],c!=null))switch(i){case`value`:o=c;break;case`defaultValue`:s=c;break;case`multiple`:r=c;default:Md(e,t,i,c,n,null)}t=o,n=s,e.multiple=!!r,t==null?n!=null&&Vt(e,!!r,n,!0):Vt(e,!!r,t,!1);return;case`textarea`:for(s in $(`invalid`,e),o=i=r=null,n)if(n.hasOwnProperty(s)&&(c=n[s],c!=null))switch(s){case`value`:r=c;break;case`defaultValue`:i=c;break;case`children`:o=c;break;case`dangerouslySetInnerHTML`:if(c!=null)throw Error(a(91));break;default:Md(e,t,s,c,n,null)}Ut(e,r,i,o);return;case`option`:for(l in n)if(n.hasOwnProperty(l)&&(r=n[l],r!=null))switch(l){case`selected`:e.selected=r&&typeof r!=`function`&&typeof r!=`symbol`;break;default:Md(e,t,l,r,n,null)}return;case`dialog`:$(`beforetoggle`,e),$(`toggle`,e),$(`cancel`,e),$(`close`,e);break;case`iframe`:case`object`:$(`load`,e);break;case`video`:case`audio`:for(r=0;r<gd.length;r++)$(gd[r],e);break;case`image`:$(`error`,e),$(`load`,e);break;case`details`:$(`toggle`,e);break;case`embed`:case`source`:case`link`:$(`error`,e),$(`load`,e);case`area`:case`base`:case`br`:case`col`:case`hr`:case`keygen`:case`meta`:case`param`:case`track`:case`wbr`:case`menuitem`:for(u in n)if(n.hasOwnProperty(u)&&(r=n[u],r!=null))switch(u){case`children`:case`dangerouslySetInnerHTML`:throw Error(a(137,t));default:Md(e,t,u,r,n,null)}return;default:if(Jt(t)){for(d in n)n.hasOwnProperty(d)&&(r=n[d],r!==void 0&&Nd(e,t,d,r,n,void 0));return}}for(c in n)n.hasOwnProperty(c)&&(r=n[c],r!=null&&Md(e,t,c,r,n,null))}function Fd(e,t,n,r){switch(t){case`div`:case`span`:case`svg`:case`path`:case`a`:case`g`:case`p`:case`li`:break;case`input`:var i=null,o=null,s=null,c=null,l=null,u=null,d=null;for(m in n){var f=n[m];if(n.hasOwnProperty(m)&&f!=null)switch(m){case`checked`:break;case`value`:break;case`defaultValue`:l=f;default:r.hasOwnProperty(m)||Md(e,t,m,null,r,f)}}for(var p in r){var m=r[p];if(f=n[p],r.hasOwnProperty(p)&&(m!=null||f!=null))switch(p){case`type`:o=m;break;case`name`:i=m;break;case`checked`:u=m;break;case`defaultChecked`:d=m;break;case`value`:s=m;break;case`defaultValue`:c=m;break;case`children`:case`dangerouslySetInnerHTML`:if(m!=null)throw Error(a(137,t));break;default:m!==f&&Md(e,t,p,m,r,f)}}Rt(e,s,c,l,u,d,o,i);return;case`select`:for(o in m=s=c=p=null,n)if(l=n[o],n.hasOwnProperty(o)&&l!=null)switch(o){case`value`:break;case`multiple`:m=l;default:r.hasOwnProperty(o)||Md(e,t,o,null,r,l)}for(i in r)if(o=r[i],l=n[i],r.hasOwnProperty(i)&&(o!=null||l!=null))switch(i){case`value`:p=o;break;case`defaultValue`:c=o;break;case`multiple`:s=o;default:o!==l&&Md(e,t,i,o,r,l)}t=c,n=s,r=m,p==null?!!r!=!!n&&(t==null?Vt(e,!!n,n?[]:``,!1):Vt(e,!!n,t,!0)):Vt(e,!!n,p,!1);return;case`textarea`:for(c in m=p=null,n)if(i=n[c],n.hasOwnProperty(c)&&i!=null&&!r.hasOwnProperty(c))switch(c){case`value`:break;case`children`:break;default:Md(e,t,c,null,r,i)}for(s in r)if(i=r[s],o=n[s],r.hasOwnProperty(s)&&(i!=null||o!=null))switch(s){case`value`:p=i;break;case`defaultValue`:m=i;break;case`children`:break;case`dangerouslySetInnerHTML`:if(i!=null)throw Error(a(91));break;default:i!==o&&Md(e,t,s,i,r,o)}Ht(e,p,m);return;case`option`:for(var h in n)if(p=n[h],n.hasOwnProperty(h)&&p!=null&&!r.hasOwnProperty(h))switch(h){case`selected`:e.selected=!1;break;default:Md(e,t,h,null,r,p)}for(l in r)if(p=r[l],m=n[l],r.hasOwnProperty(l)&&p!==m&&(p!=null||m!=null))switch(l){case`selected`:e.selected=p&&typeof p!=`function`&&typeof p!=`symbol`;break;default:Md(e,t,l,p,r,m)}return;case`img`:case`link`:case`area`:case`base`:case`br`:case`col`:case`embed`:case`hr`:case`keygen`:case`meta`:case`param`:case`source`:case`track`:case`wbr`:case`menuitem`:for(var g in n)p=n[g],n.hasOwnProperty(g)&&p!=null&&!r.hasOwnProperty(g)&&Md(e,t,g,null,r,p);for(u in r)if(p=r[u],m=n[u],r.hasOwnProperty(u)&&p!==m&&(p!=null||m!=null))switch(u){case`children`:case`dangerouslySetInnerHTML`:if(p!=null)throw Error(a(137,t));break;default:Md(e,t,u,p,r,m)}return;default:if(Jt(t)){for(var _ in n)p=n[_],n.hasOwnProperty(_)&&p!==void 0&&!r.hasOwnProperty(_)&&Nd(e,t,_,void 0,r,p);for(d in r)p=r[d],m=n[d],!r.hasOwnProperty(d)||p===m||p===void 0&&m===void 0||Nd(e,t,d,p,r,m);return}}for(var v in n)p=n[v],n.hasOwnProperty(v)&&p!=null&&!r.hasOwnProperty(v)&&Md(e,t,v,null,r,p);for(f in r)p=r[f],m=n[f],!r.hasOwnProperty(f)||p===m||p==null&&m==null||Md(e,t,f,p,r,m)}function Id(e){switch(e){case`css`:case`script`:case`font`:case`img`:case`image`:case`input`:case`link`:return!0;default:return!1}}function Ld(){if(typeof performance.getEntriesByType==`function`){for(var e=0,t=0,n=performance.getEntriesByType(`resource`),r=0;r<n.length;r++){var i=n[r],a=i.transferSize,o=i.initiatorType,s=i.duration;if(a&&s&&Id(o)){for(o=0,s=i.responseEnd,r+=1;r<n.length;r++){var c=n[r],l=c.startTime;if(l>s)break;var u=c.transferSize,d=c.initiatorType;u&&Id(d)&&(c=c.responseEnd,o+=u*(c<s?1:(s-l)/(c-l)))}if(--r,t+=8*(a+o)/(i.duration/1e3),e++,10<e)break}}if(0<e)return t/e/1e6}return navigator.connection&&(e=navigator.connection.downlink,typeof e==`number`)?e:5}var Rd=null,zd=null;function Bd(e){return e.nodeType===9?e:e.ownerDocument}function Vd(e){switch(e){case`http://www.w3.org/2000/svg`:return 1;case`http://www.w3.org/1998/Math/MathML`:return 2;default:return 0}}function Hd(e,t){if(e===0)switch(t){case`svg`:return 1;case`math`:return 2;default:return 0}return e===1&&t===`foreignObject`?0:e}function Ud(e,t){return e===`textarea`||e===`noscript`||typeof t.children==`string`||typeof t.children==`number`||typeof t.children==`bigint`||typeof t.dangerouslySetInnerHTML==`object`&&t.dangerouslySetInnerHTML!==null&&t.dangerouslySetInnerHTML.__html!=null}var Wd=null;function Gd(){var e=window.event;return e&&e.type===`popstate`?e===Wd?!1:(Wd=e,!0):(Wd=null,!1)}var Kd=typeof setTimeout==`function`?setTimeout:void 0,qd=typeof clearTimeout==`function`?clearTimeout:void 0,Jd=typeof Promise==`function`?Promise:void 0,Yd=typeof queueMicrotask==`function`?queueMicrotask:Jd===void 0?Kd:function(e){return Jd.resolve(null).then(e).catch(Xd)};function Xd(e){setTimeout(function(){throw e})}function Zd(e){return e===`head`}function Qd(e,t){var n=t,r=0;do{var i=n.nextSibling;if(e.removeChild(n),i&&i.nodeType===8)if(n=i.data,n===`/$`||n===`/&`){if(r===0){e.removeChild(i),Np(t);return}r--}else if(n===`$`||n===`$?`||n===`$~`||n===`$!`||n===`&`)r++;else if(n===`html`)pf(e.ownerDocument.documentElement);else if(n===`head`){n=e.ownerDocument.head,pf(n);for(var a=n.firstChild;a;){var o=a.nextSibling,s=a.nodeName;a[ft]||s===`SCRIPT`||s===`STYLE`||s===`LINK`&&a.rel.toLowerCase()===`stylesheet`||n.removeChild(a),a=o}}else n===`body`&&pf(e.ownerDocument.body);n=i}while(n);Np(t)}function $d(e,t){var n=e;e=0;do{var r=n.nextSibling;if(n.nodeType===1?t?(n._stashedDisplay=n.style.display,n.style.display=`none`):(n.style.display=n._stashedDisplay||``,n.getAttribute(`style`)===``&&n.removeAttribute(`style`)):n.nodeType===3&&(t?(n._stashedText=n.nodeValue,n.nodeValue=``):n.nodeValue=n._stashedText||``),r&&r.nodeType===8)if(n=r.data,n===`/$`){if(e===0)break;e--}else n!==`$`&&n!==`$?`&&n!==`$~`&&n!==`$!`||e++;n=r}while(n)}function ef(e){var t=e.firstChild;for(t&&t.nodeType===10&&(t=t.nextSibling);t;){var n=t;switch(t=t.nextSibling,n.nodeName){case`HTML`:case`HEAD`:case`BODY`:ef(n),pt(n);continue;case`SCRIPT`:case`STYLE`:continue;case`LINK`:if(n.rel.toLowerCase()===`stylesheet`)continue}e.removeChild(n)}}function tf(e,t,n,r){for(;e.nodeType===1;){var i=n;if(e.nodeName.toLowerCase()!==t.toLowerCase()){if(!r&&(e.nodeName!==`INPUT`||e.type!==`hidden`))break}else if(!r)if(t===`input`&&e.type===`hidden`){var a=i.name==null?null:``+i.name;if(i.type===`hidden`&&e.getAttribute(`name`)===a)return e}else return e;else if(!e[ft])switch(t){case`meta`:if(!e.hasAttribute(`itemprop`))break;return e;case`link`:if(a=e.getAttribute(`rel`),a===`stylesheet`&&e.hasAttribute(`data-precedence`)||a!==i.rel||e.getAttribute(`href`)!==(i.href==null||i.href===``?null:i.href)||e.getAttribute(`crossorigin`)!==(i.crossOrigin==null?null:i.crossOrigin)||e.getAttribute(`title`)!==(i.title==null?null:i.title))break;return e;case`style`:if(e.hasAttribute(`data-precedence`))break;return e;case`script`:if(a=e.getAttribute(`src`),(a!==(i.src==null?null:i.src)||e.getAttribute(`type`)!==(i.type==null?null:i.type)||e.getAttribute(`crossorigin`)!==(i.crossOrigin==null?null:i.crossOrigin))&&a&&e.hasAttribute(`async`)&&!e.hasAttribute(`itemprop`))break;return e;default:return e}if(e=cf(e.nextSibling),e===null)break}return null}function nf(e,t,n){if(t===``)return null;for(;e.nodeType!==3;)if((e.nodeType!==1||e.nodeName!==`INPUT`||e.type!==`hidden`)&&!n||(e=cf(e.nextSibling),e===null))return null;return e}function rf(e,t){for(;e.nodeType!==8;)if((e.nodeType!==1||e.nodeName!==`INPUT`||e.type!==`hidden`)&&!t||(e=cf(e.nextSibling),e===null))return null;return e}function af(e){return e.data===`$?`||e.data===`$~`}function of(e){return e.data===`$!`||e.data===`$?`&&e.ownerDocument.readyState!==`loading`}function sf(e,t){var n=e.ownerDocument;if(e.data===`$~`)e._reactRetry=t;else if(e.data!==`$?`||n.readyState!==`loading`)t();else{var r=function(){t(),n.removeEventListener(`DOMContentLoaded`,r)};n.addEventListener(`DOMContentLoaded`,r),e._reactRetry=r}}function cf(e){for(;e!=null;e=e.nextSibling){var t=e.nodeType;if(t===1||t===3)break;if(t===8){if(t=e.data,t===`$`||t===`$!`||t===`$?`||t===`$~`||t===`&`||t===`F!`||t===`F`)break;if(t===`/$`||t===`/&`)return null}}return e}var lf=null;function uf(e){e=e.nextSibling;for(var t=0;e;){if(e.nodeType===8){var n=e.data;if(n===`/$`||n===`/&`){if(t===0)return cf(e.nextSibling);t--}else n!==`$`&&n!==`$!`&&n!==`$?`&&n!==`$~`&&n!==`&`||t++}e=e.nextSibling}return null}function df(e){e=e.previousSibling;for(var t=0;e;){if(e.nodeType===8){var n=e.data;if(n===`$`||n===`$!`||n===`$?`||n===`$~`||n===`&`){if(t===0)return e;t--}else n!==`/$`&&n!==`/&`||t++}e=e.previousSibling}return null}function ff(e,t,n){switch(t=Bd(n),e){case`html`:if(e=t.documentElement,!e)throw Error(a(452));return e;case`head`:if(e=t.head,!e)throw Error(a(453));return e;case`body`:if(e=t.body,!e)throw Error(a(454));return e;default:throw Error(a(451))}}function pf(e){for(var t=e.attributes;t.length;)e.removeAttributeNode(t[0]);pt(e)}var mf=new Map,hf=new Set;function gf(e){return typeof e.getRootNode==`function`?e.getRootNode():e.nodeType===9?e:e.ownerDocument}var _f=N.d;N.d={f:vf,r:yf,D:Sf,C:Cf,L:wf,m:Tf,X:Df,S:Ef,M:Of};function vf(){var e=_f.f(),t=bu();return e||t}function yf(e){var t=ht(e);t!==null&&t.tag===5&&t.type===`form`?Cs(t):_f.r(e)}var bf=typeof document>`u`?null:document;function xf(e,t,n){var r=bf;if(r&&typeof t==`string`&&t){var i=Lt(t);i=`link[rel="`+e+`"][href="`+i+`"]`,typeof n==`string`&&(i+=`[crossorigin="`+n+`"]`),hf.has(i)||(hf.add(i),e={rel:e,crossOrigin:n,href:t},r.querySelector(i)===null&&(t=r.createElement(`link`),Pd(t,`link`,e),vt(t),r.head.appendChild(t)))}}function Sf(e){_f.D(e),xf(`dns-prefetch`,e,null)}function Cf(e,t){_f.C(e,t),xf(`preconnect`,e,t)}function wf(e,t,n){_f.L(e,t,n);var r=bf;if(r&&e&&t){var i=`link[rel="preload"][as="`+Lt(t)+`"]`;t===`image`&&n&&n.imageSrcSet?(i+=`[imagesrcset="`+Lt(n.imageSrcSet)+`"]`,typeof n.imageSizes==`string`&&(i+=`[imagesizes="`+Lt(n.imageSizes)+`"]`)):i+=`[href="`+Lt(e)+`"]`;var a=i;switch(t){case`style`:a=Af(e);break;case`script`:a=Pf(e)}mf.has(a)||(e=m({rel:`preload`,href:t===`image`&&n&&n.imageSrcSet?void 0:e,as:t},n),mf.set(a,e),r.querySelector(i)!==null||t===`style`&&r.querySelector(jf(a))||t===`script`&&r.querySelector(Ff(a))||(t=r.createElement(`link`),Pd(t,`link`,e),vt(t),r.head.appendChild(t)))}}function Tf(e,t){_f.m(e,t);var n=bf;if(n&&e){var r=t&&typeof t.as==`string`?t.as:`script`,i=`link[rel="modulepreload"][as="`+Lt(r)+`"][href="`+Lt(e)+`"]`,a=i;switch(r){case`audioworklet`:case`paintworklet`:case`serviceworker`:case`sharedworker`:case`worker`:case`script`:a=Pf(e)}if(!mf.has(a)&&(e=m({rel:`modulepreload`,href:e},t),mf.set(a,e),n.querySelector(i)===null)){switch(r){case`audioworklet`:case`paintworklet`:case`serviceworker`:case`sharedworker`:case`worker`:case`script`:if(n.querySelector(Ff(a)))return}r=n.createElement(`link`),Pd(r,`link`,e),vt(r),n.head.appendChild(r)}}}function Ef(e,t,n){_f.S(e,t,n);var r=bf;if(r&&e){var i=_t(r).hoistableStyles,a=Af(e);t||=`default`;var o=i.get(a);if(!o){var s={loading:0,preload:null};if(o=r.querySelector(jf(a)))s.loading=5;else{e=m({rel:`stylesheet`,href:e,"data-precedence":t},n),(n=mf.get(a))&&Rf(e,n);var c=o=r.createElement(`link`);vt(c),Pd(c,`link`,e),c._p=new Promise(function(e,t){c.onload=e,c.onerror=t}),c.addEventListener(`load`,function(){s.loading|=1}),c.addEventListener(`error`,function(){s.loading|=2}),s.loading|=4,Lf(o,t,r)}o={type:`stylesheet`,instance:o,count:1,state:s},i.set(a,o)}}}function Df(e,t){_f.X(e,t);var n=bf;if(n&&e){var r=_t(n).hoistableScripts,i=Pf(e),a=r.get(i);a||(a=n.querySelector(Ff(i)),a||(e=m({src:e,async:!0},t),(t=mf.get(i))&&zf(e,t),a=n.createElement(`script`),vt(a),Pd(a,`link`,e),n.head.appendChild(a)),a={type:`script`,instance:a,count:1,state:null},r.set(i,a))}}function Of(e,t){_f.M(e,t);var n=bf;if(n&&e){var r=_t(n).hoistableScripts,i=Pf(e),a=r.get(i);a||(a=n.querySelector(Ff(i)),a||(e=m({src:e,async:!0,type:`module`},t),(t=mf.get(i))&&zf(e,t),a=n.createElement(`script`),vt(a),Pd(a,`link`,e),n.head.appendChild(a)),a={type:`script`,instance:a,count:1,state:null},r.set(i,a))}}function kf(e,t,n,r){var i=(i=ce.current)?gf(i):null;if(!i)throw Error(a(446));switch(e){case`meta`:case`title`:return null;case`style`:return typeof n.precedence==`string`&&typeof n.href==`string`?(t=Af(n.href),n=_t(i).hoistableStyles,r=n.get(t),r||(r={type:`style`,instance:null,count:0,state:null},n.set(t,r)),r):{type:`void`,instance:null,count:0,state:null};case`link`:if(n.rel===`stylesheet`&&typeof n.href==`string`&&typeof n.precedence==`string`){e=Af(n.href);var o=_t(i).hoistableStyles,s=o.get(e);if(s||(i=i.ownerDocument||i,s={type:`stylesheet`,instance:null,count:0,state:{loading:0,preload:null}},o.set(e,s),(o=i.querySelector(jf(e)))&&!o._p&&(s.instance=o,s.state.loading=5),mf.has(e)||(n={rel:`preload`,as:`style`,href:n.href,crossOrigin:n.crossOrigin,integrity:n.integrity,media:n.media,hrefLang:n.hrefLang,referrerPolicy:n.referrerPolicy},mf.set(e,n),o||Nf(i,e,n,s.state))),t&&r===null)throw Error(a(528,``));return s}if(t&&r!==null)throw Error(a(529,``));return null;case`script`:return t=n.async,n=n.src,typeof n==`string`&&t&&typeof t!=`function`&&typeof t!=`symbol`?(t=Pf(n),n=_t(i).hoistableScripts,r=n.get(t),r||(r={type:`script`,instance:null,count:0,state:null},n.set(t,r)),r):{type:`void`,instance:null,count:0,state:null};default:throw Error(a(444,e))}}function Af(e){return`href="`+Lt(e)+`"`}function jf(e){return`link[rel="stylesheet"][`+e+`]`}function Mf(e){return m({},e,{"data-precedence":e.precedence,precedence:null})}function Nf(e,t,n,r){e.querySelector(`link[rel="preload"][as="style"][`+t+`]`)?r.loading=1:(t=e.createElement(`link`),r.preload=t,t.addEventListener(`load`,function(){return r.loading|=1}),t.addEventListener(`error`,function(){return r.loading|=2}),Pd(t,`link`,n),vt(t),e.head.appendChild(t))}function Pf(e){return`[src="`+Lt(e)+`"]`}function Ff(e){return`script[async]`+e}function If(e,t,n){if(t.count++,t.instance===null)switch(t.type){case`style`:var r=e.querySelector(`style[data-href~="`+Lt(n.href)+`"]`);if(r)return t.instance=r,vt(r),r;var i=m({},n,{"data-href":n.href,"data-precedence":n.precedence,href:null,precedence:null});return r=(e.ownerDocument||e).createElement(`style`),vt(r),Pd(r,`style`,i),Lf(r,n.precedence,e),t.instance=r;case`stylesheet`:i=Af(n.href);var o=e.querySelector(jf(i));if(o)return t.state.loading|=4,t.instance=o,vt(o),o;r=Mf(n),(i=mf.get(i))&&Rf(r,i),o=(e.ownerDocument||e).createElement(`link`),vt(o);var s=o;return s._p=new Promise(function(e,t){s.onload=e,s.onerror=t}),Pd(o,`link`,r),t.state.loading|=4,Lf(o,n.precedence,e),t.instance=o;case`script`:return o=Pf(n.src),(i=e.querySelector(Ff(o)))?(t.instance=i,vt(i),i):(r=n,(i=mf.get(o))&&(r=m({},n),zf(r,i)),e=e.ownerDocument||e,i=e.createElement(`script`),vt(i),Pd(i,`link`,r),e.head.appendChild(i),t.instance=i);case`void`:return null;default:throw Error(a(443,t.type))}else t.type===`stylesheet`&&!(t.state.loading&4)&&(r=t.instance,t.state.loading|=4,Lf(r,n.precedence,e));return t.instance}function Lf(e,t,n){for(var r=n.querySelectorAll(`link[rel="stylesheet"][data-precedence],style[data-precedence]`),i=r.length?r[r.length-1]:null,a=i,o=0;o<r.length;o++){var s=r[o];if(s.dataset.precedence===t)a=s;else if(a!==i)break}a?a.parentNode.insertBefore(e,a.nextSibling):(t=n.nodeType===9?n.head:n,t.insertBefore(e,t.firstChild))}function Rf(e,t){e.crossOrigin??=t.crossOrigin,e.referrerPolicy??=t.referrerPolicy,e.title??=t.title}function zf(e,t){e.crossOrigin??=t.crossOrigin,e.referrerPolicy??=t.referrerPolicy,e.integrity??=t.integrity}var Bf=null;function Vf(e,t,n){if(Bf===null){var r=new Map,i=Bf=new Map;i.set(n,r)}else i=Bf,r=i.get(n),r||(r=new Map,i.set(n,r));if(r.has(e))return r;for(r.set(e,null),n=n.getElementsByTagName(e),i=0;i<n.length;i++){var a=n[i];if(!(a[ft]||a[at]||e===`link`&&a.getAttribute(`rel`)===`stylesheet`)&&a.namespaceURI!==`http://www.w3.org/2000/svg`){var o=a.getAttribute(t)||``;o=e+o;var s=r.get(o);s?s.push(a):r.set(o,[a])}}return r}function Hf(e,t,n){e=e.ownerDocument||e,e.head.insertBefore(n,t===`title`?e.querySelector(`head > title`):null)}function Uf(e,t,n){if(n===1||t.itemProp!=null)return!1;switch(e){case`meta`:case`title`:return!0;case`style`:if(typeof t.precedence!=`string`||typeof t.href!=`string`||t.href===``)break;return!0;case`link`:if(typeof t.rel!=`string`||typeof t.href!=`string`||t.href===``||t.onLoad||t.onError)break;switch(t.rel){case`stylesheet`:return e=t.disabled,typeof t.precedence==`string`&&e==null;default:return!0}case`script`:if(t.async&&typeof t.async!=`function`&&typeof t.async!=`symbol`&&!t.onLoad&&!t.onError&&t.src&&typeof t.src==`string`)return!0}return!1}function Wf(e){return!(e.type===`stylesheet`&&!(e.state.loading&3))}function Gf(e,t,n,r){if(n.type===`stylesheet`&&(typeof r.media!=`string`||!1!==matchMedia(r.media).matches)&&!(n.state.loading&4)){if(n.instance===null){var i=Af(r.href),a=t.querySelector(jf(i));if(a){t=a._p,typeof t==`object`&&t&&typeof t.then==`function`&&(e.count++,e=Jf.bind(e),t.then(e,e)),n.state.loading|=4,n.instance=a,vt(a);return}a=t.ownerDocument||t,r=Mf(r),(i=mf.get(i))&&Rf(r,i),a=a.createElement(`link`),vt(a);var o=a;o._p=new Promise(function(e,t){o.onload=e,o.onerror=t}),Pd(a,`link`,r),n.instance=a}e.stylesheets===null&&(e.stylesheets=new Map),e.stylesheets.set(n,t),(t=n.state.preload)&&!(n.state.loading&3)&&(e.count++,n=Jf.bind(e),t.addEventListener(`load`,n),t.addEventListener(`error`,n))}}var Kf=0;function qf(e,t){return e.stylesheets&&e.count===0&&Xf(e,e.stylesheets),0<e.count||0<e.imgCount?function(n){var r=setTimeout(function(){if(e.stylesheets&&Xf(e,e.stylesheets),e.unsuspend){var t=e.unsuspend;e.unsuspend=null,t()}},6e4+t);0<e.imgBytes&&Kf===0&&(Kf=62500*Ld());var i=setTimeout(function(){if(e.waitingForImages=!1,e.count===0&&(e.stylesheets&&Xf(e,e.stylesheets),e.unsuspend)){var t=e.unsuspend;e.unsuspend=null,t()}},(e.imgBytes>Kf?50:800)+t);return e.unsuspend=n,function(){e.unsuspend=null,clearTimeout(r),clearTimeout(i)}}:null}function Jf(){if(this.count--,this.count===0&&(this.imgCount===0||!this.waitingForImages)){if(this.stylesheets)Xf(this,this.stylesheets);else if(this.unsuspend){var e=this.unsuspend;this.unsuspend=null,e()}}}var Yf=null;function Xf(e,t){e.stylesheets=null,e.unsuspend!==null&&(e.count++,Yf=new Map,t.forEach(Zf,e),Yf=null,Jf.call(e))}function Zf(e,t){if(!(t.state.loading&4)){var n=Yf.get(e);if(n)var r=n.get(null);else{n=new Map,Yf.set(e,n);for(var i=e.querySelectorAll(`link[data-precedence],style[data-precedence]`),a=0;a<i.length;a++){var o=i[a];(o.nodeName===`LINK`||o.getAttribute(`media`)!==`not all`)&&(n.set(o.dataset.precedence,o),r=o)}r&&n.set(null,r)}i=t.instance,o=i.getAttribute(`data-precedence`),a=n.get(o)||r,a===r&&n.set(null,i),n.set(o,i),this.count++,r=Jf.bind(this),i.addEventListener(`load`,r),i.addEventListener(`error`,r),a?a.parentNode.insertBefore(i,a.nextSibling):(e=e.nodeType===9?e.head:e,e.insertBefore(i,e.firstChild)),t.state.loading|=4}}var Qf={$$typeof:S,Provider:null,Consumer:null,_currentValue:P,_currentValue2:P,_threadCount:0};function $f(e,t,n,r,i,a,o,s,c){this.tag=1,this.containerInfo=e,this.pingCache=this.current=this.pendingChildren=null,this.timeoutHandle=-1,this.callbackNode=this.next=this.pendingContext=this.context=this.cancelPendingCommit=null,this.callbackPriority=0,this.expirationTimes=Je(-1),this.entangledLanes=this.shellSuspendCounter=this.errorRecoveryDisabledLanes=this.expiredLanes=this.warmLanes=this.pingedLanes=this.suspendedLanes=this.pendingLanes=0,this.entanglements=Je(0),this.hiddenUpdates=Je(null),this.identifierPrefix=r,this.onUncaughtError=i,this.onCaughtError=a,this.onRecoverableError=o,this.pooledCache=null,this.pooledCacheLanes=0,this.formState=c,this.incompleteTransitions=new Map}function ep(e,t,n,r,i,a,o,s,c,l,u,d){return e=new $f(e,t,n,o,c,l,u,d,s),t=1,!0===a&&(t|=24),a=ai(3,null,null,t),e.current=a,a.stateNode=e,t=aa(),t.refCount++,e.pooledCache=t,t.refCount++,a.memoizedState={element:r,isDehydrated:n,cache:t},Ra(a),e}function tp(e){return e?(e=ri,e):ri}function np(e,t,n,r,i,a){i=tp(i),r.context===null?r.context=i:r.pendingContext=i,r=Ba(t),r.payload={element:n},a=a===void 0?null:a,a!==null&&(r.callback=a),n=Va(e,r,t),n!==null&&(hu(n,e,t),Ha(n,e,t))}function rp(e,t){if(e=e.memoizedState,e!==null&&e.dehydrated!==null){var n=e.retryLane;e.retryLane=n!==0&&n<t?n:t}}function ip(e,t){rp(e,t),(e=e.alternate)&&rp(e,t)}function ap(e){if(e.tag===13||e.tag===31){var t=ei(e,67108864);t!==null&&hu(t,e,67108864),ip(e,67108864)}}function op(e){if(e.tag===13||e.tag===31){var t=pu();t=et(t);var n=ei(e,t);n!==null&&hu(n,e,t),ip(e,t)}}var sp=!0;function cp(e,t,n,r){var i=M.T;M.T=null;var a=N.p;try{N.p=2,up(e,t,n,r)}finally{N.p=a,M.T=i}}function lp(e,t,n,r){var i=M.T;M.T=null;var a=N.p;try{N.p=8,up(e,t,n,r)}finally{N.p=a,M.T=i}}function up(e,t,n,r){if(sp){var i=dp(r);if(i===null)Cd(e,t,r,fp,n),Cp(e,r);else if(Tp(i,e,t,n,r))r.stopPropagation();else if(Cp(e,r),t&4&&-1<Sp.indexOf(e)){for(;i!==null;){var a=ht(i);if(a!==null)switch(a.tag){case 3:if(a=a.stateNode,a.current.memoizedState.isDehydrated){var o=Ue(a.pendingLanes);if(o!==0){var s=a;for(s.pendingLanes|=2,s.entangledLanes|=2;o;){var c=1<<31-Ie(o);s.entanglements[1]|=c,o&=~c}nd(a),!(Il&6)&&(tu=we()+500,rd(0,!1))}}break;case 31:case 13:s=ei(a,2),s!==null&&hu(s,a,2),bu(),ip(a,2)}if(a=dp(r),a===null&&Cd(e,t,r,fp,n),a===i)break;i=a}i!==null&&r.stopPropagation()}else Cd(e,t,r,null,n)}}function dp(e){return e=en(e),pp(e)}var fp=null;function pp(e){if(fp=null,e=mt(e),e!==null){var t=s(e);if(t===null)e=null;else{var n=t.tag;if(n===13){if(e=c(t),e!==null)return e;e=null}else if(n===31){if(e=l(t),e!==null)return e;e=null}else if(n===3){if(t.stateNode.current.memoizedState.isDehydrated)return t.tag===3?t.stateNode.containerInfo:null;e=null}else t!==e&&(e=null)}}return fp=e,null}function mp(e){switch(e){case`beforetoggle`:case`cancel`:case`click`:case`close`:case`contextmenu`:case`copy`:case`cut`:case`auxclick`:case`dblclick`:case`dragend`:case`dragstart`:case`drop`:case`focusin`:case`focusout`:case`input`:case`invalid`:case`keydown`:case`keypress`:case`keyup`:case`mousedown`:case`mouseup`:case`paste`:case`pause`:case`play`:case`pointercancel`:case`pointerdown`:case`pointerup`:case`ratechange`:case`reset`:case`resize`:case`seeked`:case`submit`:case`toggle`:case`touchcancel`:case`touchend`:case`touchstart`:case`volumechange`:case`change`:case`selectionchange`:case`textInput`:case`compositionstart`:case`compositionend`:case`compositionupdate`:case`beforeblur`:case`afterblur`:case`beforeinput`:case`blur`:case`fullscreenchange`:case`focus`:case`hashchange`:case`popstate`:case`select`:case`selectstart`:return 2;case`drag`:case`dragenter`:case`dragexit`:case`dragleave`:case`dragover`:case`mousemove`:case`mouseout`:case`mouseover`:case`pointermove`:case`pointerout`:case`pointerover`:case`scroll`:case`touchmove`:case`wheel`:case`mouseenter`:case`mouseleave`:case`pointerenter`:case`pointerleave`:return 8;case`message`:switch(Te()){case Ee:return 2;case De:return 8;case Oe:case ke:return 32;case Ae:return 268435456;default:return 32}default:return 32}}var hp=!1,gp=null,_p=null,vp=null,yp=new Map,bp=new Map,xp=[],Sp=`mousedown mouseup touchcancel touchend touchstart auxclick dblclick pointercancel pointerdown pointerup dragend dragstart drop compositionend compositionstart keydown keypress keyup input textInput copy cut paste click change contextmenu reset`.split(` `);function Cp(e,t){switch(e){case`focusin`:case`focusout`:gp=null;break;case`dragenter`:case`dragleave`:_p=null;break;case`mouseover`:case`mouseout`:vp=null;break;case`pointerover`:case`pointerout`:yp.delete(t.pointerId);break;case`gotpointercapture`:case`lostpointercapture`:bp.delete(t.pointerId)}}function wp(e,t,n,r,i,a){return e===null||e.nativeEvent!==a?(e={blockedOn:t,domEventName:n,eventSystemFlags:r,nativeEvent:a,targetContainers:[i]},t!==null&&(t=ht(t),t!==null&&ap(t)),e):(e.eventSystemFlags|=r,t=e.targetContainers,i!==null&&t.indexOf(i)===-1&&t.push(i),e)}function Tp(e,t,n,r,i){switch(t){case`focusin`:return gp=wp(gp,e,t,n,r,i),!0;case`dragenter`:return _p=wp(_p,e,t,n,r,i),!0;case`mouseover`:return vp=wp(vp,e,t,n,r,i),!0;case`pointerover`:var a=i.pointerId;return yp.set(a,wp(yp.get(a)||null,e,t,n,r,i)),!0;case`gotpointercapture`:return a=i.pointerId,bp.set(a,wp(bp.get(a)||null,e,t,n,r,i)),!0}return!1}function Ep(e){var t=mt(e.target);if(t!==null){var n=s(t);if(n!==null){if(t=n.tag,t===13){if(t=c(n),t!==null){e.blockedOn=t,rt(e.priority,function(){op(n)});return}}else if(t===31){if(t=l(n),t!==null){e.blockedOn=t,rt(e.priority,function(){op(n)});return}}else if(t===3&&n.stateNode.current.memoizedState.isDehydrated){e.blockedOn=n.tag===3?n.stateNode.containerInfo:null;return}}}e.blockedOn=null}function Dp(e){if(e.blockedOn!==null)return!1;for(var t=e.targetContainers;0<t.length;){var n=dp(e.nativeEvent);if(n===null){n=e.nativeEvent;var r=new n.constructor(n.type,n);$t=r,n.target.dispatchEvent(r),$t=null}else return t=ht(n),t!==null&&ap(t),e.blockedOn=n,!1;t.shift()}return!0}function Op(e,t,n){Dp(e)&&n.delete(t)}function kp(){hp=!1,gp!==null&&Dp(gp)&&(gp=null),_p!==null&&Dp(_p)&&(_p=null),vp!==null&&Dp(vp)&&(vp=null),yp.forEach(Op),bp.forEach(Op)}function Ap(e,n){e.blockedOn===n&&(e.blockedOn=null,hp||(hp=!0,t.unstable_scheduleCallback(t.unstable_NormalPriority,kp)))}var jp=null;function Mp(e){jp!==e&&(jp=e,t.unstable_scheduleCallback(t.unstable_NormalPriority,function(){jp===e&&(jp=null);for(var t=0;t<e.length;t+=3){var n=e[t],r=e[t+1],i=e[t+2];if(typeof r!=`function`){if(pp(r||n)===null)continue;break}var a=ht(n);a!==null&&(e.splice(t,3),t-=3,Ss(a,{pending:!0,data:i,method:n.method,action:r},r,i))}}))}function Np(e){function t(t){return Ap(t,e)}gp!==null&&Ap(gp,e),_p!==null&&Ap(_p,e),vp!==null&&Ap(vp,e),yp.forEach(t),bp.forEach(t);for(var n=0;n<xp.length;n++){var r=xp[n];r.blockedOn===e&&(r.blockedOn=null)}for(;0<xp.length&&(n=xp[0],n.blockedOn===null);)Ep(n),n.blockedOn===null&&xp.shift();if(n=(e.ownerDocument||e).$$reactFormReplay,n!=null)for(r=0;r<n.length;r+=3){var i=n[r],a=n[r+1],o=i[ot]||null;if(typeof a==`function`)o||Mp(n);else if(o){var s=null;if(a&&a.hasAttribute(`formAction`)){if(i=a,o=a[ot]||null)s=o.formAction;else if(pp(i)!==null)continue}else s=o.action;typeof s==`function`?n[r+1]=s:(n.splice(r,3),r-=3),Mp(n)}}}function Pp(){function e(e){e.canIntercept&&e.info===`react-transition`&&e.intercept({handler:function(){return new Promise(function(e){return i=e})},focusReset:`manual`,scroll:`manual`})}function t(){i!==null&&(i(),i=null),r||setTimeout(n,20)}function n(){if(!r&&!navigation.transition){var e=navigation.currentEntry;e&&e.url!=null&&navigation.navigate(e.url,{state:e.getState(),info:`react-transition`,history:`replace`})}}if(typeof navigation==`object`){var r=!1,i=null;return navigation.addEventListener(`navigate`,e),navigation.addEventListener(`navigatesuccess`,t),navigation.addEventListener(`navigateerror`,t),setTimeout(n,100),function(){r=!0,navigation.removeEventListener(`navigate`,e),navigation.removeEventListener(`navigatesuccess`,t),navigation.removeEventListener(`navigateerror`,t),i!==null&&(i(),i=null)}}}function Fp(e){this._internalRoot=e}Ip.prototype.render=Fp.prototype.render=function(e){var t=this._internalRoot;if(t===null)throw Error(a(409));var n=t.current;np(n,pu(),e,t,null,null)},Ip.prototype.unmount=Fp.prototype.unmount=function(){var e=this._internalRoot;if(e!==null){this._internalRoot=null;var t=e.containerInfo;np(e.current,2,null,e,null,null),bu(),t[st]=null}};function Ip(e){this._internalRoot=e}Ip.prototype.unstable_scheduleHydration=function(e){if(e){var t=nt();e={blockedOn:null,target:e,priority:t};for(var n=0;n<xp.length&&t!==0&&t<xp[n].priority;n++);xp.splice(n,0,e),n===0&&Ep(e)}};var Lp=r.version;if(Lp!==`19.2.8`)throw Error(a(527,Lp,`19.2.8`));N.findDOMNode=function(e){var t=e._reactInternals;if(t===void 0)throw typeof e.render==`function`?Error(a(188)):(e=Object.keys(e).join(`,`),Error(a(268,e)));return e=f(t),e=e===null?null:p(e),e=e===null?null:e.stateNode,e};var Rp={bundleType:0,version:`19.2.8`,rendererPackageName:`react-dom`,currentDispatcherRef:M,reconcilerVersion:`19.2.8`};if(typeof __REACT_DEVTOOLS_GLOBAL_HOOK__<`u`){var zp=__REACT_DEVTOOLS_GLOBAL_HOOK__;if(!zp.isDisabled&&zp.supportsFiber)try{Ne=zp.inject(Rp),Pe=zp}catch{}}e.createRoot=function(e,t){if(!o(e))throw Error(a(299));var n=!1,r=``,i=Us,s=Ws,c=Gs;return t!=null&&(!0===t.unstable_strictMode&&(n=!0),t.identifierPrefix!==void 0&&(r=t.identifierPrefix),t.onUncaughtError!==void 0&&(i=t.onUncaughtError),t.onCaughtError!==void 0&&(s=t.onCaughtError),t.onRecoverableError!==void 0&&(c=t.onRecoverableError)),t=ep(e,1,!1,null,null,n,r,null,i,s,c,Pp),e[st]=t.current,xd(e),new Fp(t)}})),N=t(((e,t)=>{function n(){if(!(typeof __REACT_DEVTOOLS_GLOBAL_HOOK__>`u`||typeof __REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE!=`function`))try{__REACT_DEVTOOLS_GLOBAL_HOOK__.checkDCE(n)}catch(e){console.error(e)}}n(),t.exports=M()})),P=e(n(),1),ie=N(),F={wrapper:`_wrapper_1xh50_2`,header:`_header_1xh50_10`,headerActions:`_headerActions_1xh50_21`,title:`_title_1xh50_27`,panelGroup:`_panelGroup_1xh50_36`,panelToggle:`_panelToggle_1xh50_43`,helpToggle:`_helpToggle_1xh50_66`,helpButtonWrapper:`_helpButtonWrapper_1xh50_93`,helpTogglePulsing:`_helpTogglePulsing_1xh50_97`,helpPulse:`_helpPulse_1xh50_1`,helpHint:`_helpHint_1xh50_112`,helpHintFading:`_helpHintFading_1xh50_139`,helpHintKbd:`_helpHintKbd_1xh50_144`,resizeHandle:`_resizeHandle_1xh50_153`},ae=e=>{try{return!new DOMParser().parseFromString(e.trim(),`text/xml`).querySelector(`parsererror`)}catch{return!1}},I=e=>{try{return JSON.parse(e),!0}catch{return!1}},L=e=>e.trim()?I(e)?{valid:!0,error:null,type:`json`}:ae(e)?{valid:!0,error:null,type:`xml`}:{valid:!1,error:`Invalid JSON/XML format`,type:null}:{valid:!0,error:null,type:null},oe=e=>{try{let t=JSON.parse(e);return JSON.stringify(t,null,2)}catch{return e}},se=()=>{let[e,t]=(0,P.useState)([]),n=(0,P.useRef)(0),r=(0,P.useRef)(new Set);return(0,P.useEffect)(()=>()=>{r.current.forEach(clearTimeout)},[]),{toasts:e,addToast:(0,P.useCallback)((e,i=`info`,a)=>{let o=++n.current;t(t=>[...t,{id:o,message:e,type:i,action:a?.action}]);let s=setTimeout(()=>{r.current.delete(s),t(e=>e.filter(e=>e.id!==o))},a?.durationMs??3e3);r.current.add(s)},[]),removeToast:(0,P.useCallback)(e=>{t(t=>t.filter(t=>t.id!==e))},[])}},ce=(e,t)=>{let n=(0,P.useCallback)(()=>{try{let n=window.localStorage.getItem(e);return n?JSON.parse(n):t}catch{return t}},[e]),[r,i]=(0,P.useState)(n);return(0,P.useEffect)(()=>{i(n())},[e]),(0,P.useEffect)(()=>{try{window.localStorage.setItem(e,JSON.stringify(r))}catch(t){console.error(`Error setting localStorage key "${e}":`,t)}},[e,r]),(0,P.useEffect)(()=>{let t=t=>{(t.key===e||t.key===null)&&i(n())};return window.addEventListener(`storage`,t),()=>window.removeEventListener(`storage`,t)},[e,n]),(0,P.useEffect)(()=>{let e=()=>i(n());return window.addEventListener(`focus`,e),document.addEventListener(`visibilitychange`,e),()=>{window.removeEventListener(`focus`,e),document.removeEventListener(`visibilitychange`,e)}},[n]),[r,i]},le=2e4,ue=[{path:`/json-path`,label:`JSON-Path`,title:`JSON-Path Playground`,wsPath:`/ws/json/path`,storageKeyPayload:`jsonpath-last-payload`,storageKeyHistory:`jsonpath-command-history`,storageKeyTab:`jsonpath-right-tab`,storageKeyHelpTopic:`jsonpath-help-topic`,supportsUpload:!0,supportsHelp:!0,helpContentProfile:`json-path`,tabs:[`payload`]},{path:`/`,label:`Minigraph`,title:`Minigraph Playground`,wsPath:`/ws/graph/playground`,storageKeyPayload:`minigraph-last-payload`,storageKeyHistory:`minigraph-command-history`,storageKeyTab:`minigraph-right-tab`,storageKeySavedGraphs:`minigraph-saved-graphs`,storageKeyHelpTopic:`minigraph-help-topic`,supportsClipboard:!0,supportsHelp:!0,helpContentProfile:`minigraph`,supportsAuthoring:!0,supportsGraphRun:!0,supportsSessionCollaboration:!0,tabs:[`graph`,`graph-data`]}],R={json_simple:JSON.stringify({name:`John Doe`,age:30,city:`New York`},null,2),json_nested:JSON.stringify({user:{name:`Jane Smith`,profile:{email:`jane@example.com`,address:{city:`San Francisco`,country:`USA`}}}},null,2),json_array:JSON.stringify([{id:1,name:`Item 1`,status:`active`},{id:2,name:`Item 2`,status:`pending`},{id:3,name:`Item 3`,status:`inactive`}],null,2),xml_simple:`<?xml version="1.0" encoding="UTF-8"?>
<person>
  <name>John Doe</name>
  <age>30</age>
  <city>New York</city>
</person>`,xml_nested:`<?xml version="1.0" encoding="UTF-8"?>
<user>
  <name>Jane Smith</name>
  <profile>
    <email>jane@example.com</email>
    <address>
      <city>San Francisco</city>
      <country>USA</country>
    </address>
  </profile>
</user>`,xml_array:`<?xml version="1.0" encoding="UTF-8"?>
<items>
  <item>
    <id>1</id>
    <name>Item 1</name>
    <status>active</status>
  </item>
  <item>
    <id>2</id>
    <name>Item 2</name>
    <status>pending</status>
  </item>
  <item>
    <id>3</id>
    <name>Item 3</name>
    <status>inactive</status>
  </item>
</items>`};function de(e){return`ws://${window.location.host}${e}`}var z=O();function fe(e,t,n,r){let i=e[t]??{phase:`idle`,connectionEpoch:null,messages:[]},a=[...i.messages,{id:n,raw:r}];return a.length>200&&a.shift(),{...e,[t]:{...i,messages:a}}}function pe(e,t){let n=e[t.path]??{phase:`idle`,connectionEpoch:null,messages:[]};switch(t.type){case`CONNECTING`:return{...e,[t.path]:{...n,phase:`connecting`}};case`CONNECTED`:return fe({...e,[t.path]:{...n,phase:`connected`,connectionEpoch:t.id}},t.path,t.id,t.msg);case`MESSAGE_RECEIVED`:return fe(e,t.path,t.id,t.msg);case`DISCONNECTED`:return fe({...e,[t.path]:{...n,phase:`idle`}},t.path,t.id,t.msg);case`CONNECT_ERROR`:return{...e,[t.path]:{...n,phase:`idle`}};case`CLEAR_MESSAGES`:return{...e,[t.path]:{...n,messages:[]}};default:return e}}var me=(0,P.createContext)(null);function he({children:e}){let[t,n]=(0,P.useReducer)(pe,{}),r=(0,P.useRef)({}),i=(0,P.useRef)({}),a=(0,P.useRef)({});(0,P.useEffect)(()=>()=>{Object.entries(r.current).forEach(([e,t])=>{t?.close();let n=i.current[e];n&&clearInterval(n)})},[]);let o=e=>de(e),s=e=>(a.current[e]=(a.current[e]??0)+1,a.current[e]),c=()=>{let e=new Date,t=e=>String(e).padStart(2,`0`);return`${t(e.getHours())}:${t(e.getMinutes())}:${t(e.getSeconds())}`},l=(e,t)=>JSON.stringify({type:e,message:t,time:c()}),u=e=>{try{let t=JSON.parse(e);if(typeof t==`object`&&t){let e=t.type;return e===`ping`||e===`pong`}}catch{}return!1},d=(0,P.useCallback)((e,t)=>{if(!window.WebSocket){t?.(`WebSocket not supported by your browser`,`error`);return}let a=r.current[e];if(a&&(a.readyState===WebSocket.OPEN||a.readyState===WebSocket.CONNECTING)){t?.(`Already connected`,`error`);return}n({type:`CONNECTING`,path:e});let c=new WebSocket(o(e));r.current[e]=c,c.onopen=()=>{n({type:`CONNECTED`,path:e,id:s(e),msg:l(`info`,`connected`)}),t?.(`Connected to WebSocket`,`success`),c.send(JSON.stringify({type:`welcome`})),i.current[e]=setInterval(()=>{c.readyState===WebSocket.OPEN&&c.send(l(`ping`,`keep alive`))},le)},c.onmessage=t=>{u(t.data)||n({type:`MESSAGE_RECEIVED`,path:e,id:s(e),msg:t.data})},c.onerror=()=>{n({type:`CONNECT_ERROR`,path:e})},c.onclose=a=>{let o=i.current[e];o&&(clearInterval(o),i.current[e]=null),n({type:`DISCONNECTED`,path:e,id:s(e),msg:l(`info`,`disconnected - (${a.code}) ${a.reason}`)}),t?.(`Disconnected from WebSocket`,`info`),r.current[e]===c&&(r.current[e]=null)}},[]),f=(0,P.useCallback)(e=>{let t=r.current[e];t?t.close():n({type:`MESSAGE_RECEIVED`,path:e,id:s(e),msg:l(`error`,`already disconnected`)})},[]);(0,P.useEffect)(()=>(ue.forEach(e=>{d(e.wsPath)}),()=>{ue.forEach(e=>{let t=r.current[e.wsPath];t&&t.close()})}),[]);let p=(0,P.useCallback)((e,t)=>{let n=r.current[e];return n&&n.readyState===WebSocket.OPEN?(n.send(t),!0):!1},[]),m=(0,P.useCallback)((e,t)=>{n({type:`MESSAGE_RECEIVED`,path:e,id:s(e),msg:t})},[]),h=(0,P.useCallback)(e=>{n({type:`CLEAR_MESSAGES`,path:e})},[]),[g,_]=(0,P.useState)({}),v=(0,P.useCallback)((e,t)=>{_(n=>{if(t===null){let t={...n};return delete t[e],t}return{...n,[e]:t}})},[]),y=(0,P.useCallback)(e=>g[e]??null,[g]),b=(0,P.useCallback)(e=>{let t=g[e]??null;return t!==null&&_(t=>{let n={...t};return delete n[e],n}),t},[g]),x=(0,P.useCallback)(e=>t[e]??{phase:`idle`,connectionEpoch:null,messages:[]},[t]),S=(0,P.useMemo)(()=>({getSlot:x,connect:d,disconnect:f,send:p,appendMessage:m,clearMessages:h,setPendingPayload:v,peekPendingPayload:y,takePendingPayload:b}),[x,d,f,p,m,h,v,y,b]);return(0,z.jsx)(me.Provider,{value:S,children:e})}function ge(){let e=(0,P.useContext)(me);if(!e)throw Error(`useWebSocketContext must be used inside <WebSocketProvider>`);return e}var _e=e=>{try{let t=JSON.parse(e);return{type:t.type||`info`,message:t.message||e,time:t.time,raw:e}}catch{return{type:`raw`,message:e,time:null,raw:e}}},ve=e=>({info:`ℹ️`,error:`❌`,ping:`🔄`,welcome:`👋`,raw:``})[e]??`•`,ye=e=>{try{let t=JSON.parse(e);if(typeof t==`object`&&t)return{isJSON:!0,data:t}}catch{}return{isJSON:!1,data:null}};function be(e){if(!e.includes(`Graph exported to `))return null;let t=Ce(e);if(!t)return null;let n=t.split(`/`)[4];return n?{graphName:n,apiPath:t}:null}function xe(e){return e.includes(`Invalid filename`)?{reason:`invalid-name`}:e.includes(`Expect root node name`)?{reason:`root-name-conflict`}:null}function Se(e){let t=ye(e);return t.isJSON?(t.data.type,!1):!0}function Ce(e){let t=e.match(/\/api\/graph\/model\/([^\s'"]+)/);return t?t[0]:null}function we(e){return Se(e)?Ce(e)!==null:!1}function Te(e){let t=e.match(/\/api\/json\/content\/([\w-]+)/);return t?t[0]:null}function Ee(e){let t=e.match(/Large payload \((\d+)\)\s*->\s*GET\s+(\/api\/inspect\/[^\s]+)/i);if(!t)return null;let n=parseInt(t[1],10),r=t[2];return{apiPath:r,byteSize:n,filename:`${r.split(`/`).filter(Boolean).pop()??`payload`}.json`}}function De(e){let t=e.match(/You may upload .*?->\s*POST\s+(\/api\/mock\/[\w-]+)/i);return t?t[1]:null}function Oe(e){if(!e.startsWith(`> `))return!1;let t=e.slice(2).trim().toLowerCase();return t===`help`||t.startsWith(`help `)?!0:t.startsWith(`describe `)?!t.slice(9).trim().startsWith(`graph`):!1}function ke(e){if(!e.startsWith(`> `)||!e.slice(2).trimStart().toLowerCase().startsWith(`import graph from `))return null;let t=e.slice(2).trimStart().slice(18).trim();return t.length>0?t:null}var Ae=/^node ([A-Za-z0-9_-]+) created$/i,je=/^node ([A-Za-z0-9_-]+) already exists$/i,Me=/^node ([A-Za-z0-9_-]+) updated$/i,Ne=/^node ([A-Za-z0-9_-]+) deleted$/i,Pe=/^node ([A-Za-z0-9_-]+) connected to ([A-Za-z0-9_-]+)$/i,Fe=/^node ([A-Za-z0-9_-]+) not found$/i,Ie=/^Source and target nodes must be different$/i,Le=/^Syntax: connect \{node-A\} to \{node-B\} with \{relation\}$/i,Re=/^ERROR: (.+)$/;function ze(e){let t=e.trim();if(t.startsWith(`> `))return null;let n=t.match(Ae);if(n)return{status:`accepted`,action:`create-node`,alias:n[1],targetAlias:null,message:t};let r=t.match(je);if(r)return{status:`rejected`,action:`create-node`,alias:r[1],targetAlias:null,message:t};let i=t.match(Me);if(i)return{status:`accepted`,action:`edit-node`,alias:i[1],targetAlias:null,message:t};let a=t.match(Ne);if(a)return{status:`accepted`,action:`delete-node`,alias:a[1],targetAlias:null,message:t};let o=t.match(Pe);if(o)return{status:`accepted`,action:`create-connection`,alias:o[1],targetAlias:o[2],message:t};let s=t.match(Fe);return s?{status:`rejected`,action:null,alias:s[1],targetAlias:null,message:t}:Ie.test(t)||Le.test(t)?{status:`rejected`,action:`create-connection`,alias:null,targetAlias:null,message:t}:t.match(Re)?{status:`error`,action:null,alias:null,targetAlias:null,message:t}:null}function Be(e){if(!Se(e)||e.startsWith(`> `)||we(e))return null;let t=e.toLowerCase();return t.includes(`graph model imported as draft`)?`import-graph`:t.includes(` -> `)&&t.includes(`removed`)||t.startsWith(`node `)&&(t.includes(` created`)||t.includes(` updated`)||t.includes(` deleted`)||t.includes(` connected to `)||t.includes(` imported from `)||t.includes(` overwritten by node from `))?`node-mutation`:null}var Ve={command:``,historyIndex:-1,draftCommand:``};function He(e,t){switch(t.type){case`SET_COMMAND`:return{...e,command:t.value,historyIndex:-1,draftCommand:``};case`CLEAR_COMMAND`:return{...e,command:``,historyIndex:-1,draftCommand:``};case`SET_HISTORY_INDEX`:return{...e,historyIndex:t.index,command:t.command};case`ENTER_HISTORY`:return{...e,historyIndex:0,command:t.command,draftCommand:e.command};case`EXIT_HISTORY`:return{...e,historyIndex:-1,command:e.draftCommand,draftCommand:``};default:return e}}function Ue({wsPath:e,storageKeyHistory:t,payload:n,addToast:r,bus:i,handleLocalCommand:a}){let o=ge(),{phase:s,connectionEpoch:c,messages:l}=o.getSlot(e),u=s===`connected`,d=s===`connecting`,[f,p]=(0,P.useReducer)(He,Ve),{command:m,historyIndex:h}=f,[g,_]=ce(t,[]),v=(0,P.useRef)(null),y=(0,P.useRef)(!1);(0,P.useEffect)(()=>{v.current&&(v.current.scrollTop=v.current.scrollHeight)},[l]);let b=(0,P.useCallback)(()=>{o.connect(e,r)},[o,e,r]),x=(0,P.useCallback)(()=>{o.disconnect(e)},[o,e]),S=(0,P.useCallback)(()=>{if(s!==`connected`)return;let t=m.trim();if(t.length!==0){if(a?.(t)===!0){g[0]!==t&&_(e=>[t,...e].slice(0,50)),o.appendMessage(e,`> `+t),p({type:`CLEAR_COMMAND`});return}o.send(e,t),g[0]!==t&&_(e=>[t,...e].slice(0,50)),t===`load`&&(n.length===0?o.appendMessage(e,`ERROR: please paste JSON/XML payload in input text area`):o.send(e,n)),p({type:`CLEAR_COMMAND`})}},[o,e,s,m,n,g,_,a]),C=(0,P.useCallback)(e=>{if(e.key===`ArrowUp`){if(e.preventDefault(),g.length===0)return;if(h===-1)p({type:`ENTER_HISTORY`,command:g[0]});else if(h<g.length-1){let e=h+1;p({type:`SET_HISTORY_INDEX`,index:e,command:g[e]})}}else if(e.key===`ArrowDown`)if(e.preventDefault(),h<=0)h===0&&p({type:`EXIT_HISTORY`});else{let e=h-1;p({type:`SET_HISTORY_INDEX`,index:e,command:g[e]})}},[g,h]);(0,P.useEffect)(()=>{if(i)return i.on(`upload.contentPath`,t=>{if(!y.current)return;if(y.current=!1,n.length===0){o.appendMessage(e,`ERROR: please paste JSON/XML payload in the input text area`);return}let i;try{i=JSON.stringify(JSON.parse(n))}catch{o.appendMessage(e,`ERROR: payload is not valid JSON — cannot upload`);return}fetch(t.uploadPath,{method:`POST`,headers:{"Content-Type":`application/json`},body:i}).then(e=>{if(!e.ok)throw Error(`HTTP ${e.status}`);r(`Payload uploaded successfully`,`success`)}).catch(t=>{o.appendMessage(e,`ERROR: upload failed — ${t.message}`),r(`Upload failed: ${t.message}`,`error`)})})},[i,n,e,o,r]),(0,P.useEffect)(()=>{if(i||!y.current||l.length===0)return;let t=l[l.length-1].raw,a=Te(t);if(!a)return;if(y.current=!1,n.length===0){o.appendMessage(e,`ERROR: please paste JSON/XML payload in the input text area`);return}let s;try{s=JSON.stringify(JSON.parse(n))}catch{o.appendMessage(e,`ERROR: payload is not valid JSON — cannot upload`);return}fetch(a,{method:`POST`,headers:{"Content-Type":`application/json`},body:s}).then(e=>{if(!e.ok)throw Error(`HTTP ${e.status}`);r(`Payload uploaded successfully`,`success`)}).catch(t=>{o.appendMessage(e,`ERROR: upload failed — ${t.message}`),r(`Upload failed: ${t.message}`,`error`)})},[i,l,n,e,o,r]);let w=(0,P.useCallback)(()=>{if(s===`connected`){if(n.length===0){r(`Nothing to upload — paste a JSON payload first`,`error`);return}y.current=!0,o.send(e,`upload`)}},[o,e,s,n,r]),ee=(0,P.useCallback)(t=>s===`connected`&&o.send(e,t),[o,e,s]),T=(0,P.useCallback)(()=>{navigator.clipboard.writeText(l.map(e=>e.raw).join(`
`)),r(`Console copied to clipboard!`,`success`)},[l,r]),E=(0,P.useCallback)(()=>{o.clearMessages(e),r(`Console cleared`,`info`)},[o,e,r]),D=(0,P.useCallback)(t=>{o.appendMessage(e,t)},[o,e]);return{connected:u,connecting:d,connectionEpoch:c,messages:l,command:m,setCommand:(0,P.useCallback)(e=>p({type:`SET_COMMAND`,value:e}),[]),connect:b,disconnect:x,sendCommand:S,handleKeyDown:C,consoleRef:v,copyMessages:T,clearMessages:E,uploadPayload:w,sendRawText:ee,appendMessage:D,history:g}}function We(e){let[t,n]=(0,P.useState)(()=>window.matchMedia(e).matches);return(0,P.useEffect)(()=>{let t=window.matchMedia(e),r=e=>n(e.matches);return t.addEventListener(`change`,r),()=>t.removeEventListener(`change`,r)},[e]),t}function Ge(e){if(typeof e!=`object`||!e)return!1;let t=e;return Array.isArray(t.nodes)}function Ke(e,t,n){let r=t.includes(n)?n:t[0]??`graph`;return typeof e==`string`&&t.includes(e)?e:r}function qe(e){return Ge(e)&&e.nodes.length>0?e:null}function Je(e,t,n,r,i){let[a,o]=(0,P.useState)(null),[s,c]=ce(i,n),l=Ke(s,r,n),[u,d]=(0,P.useState)(!1),f=(0,P.useCallback)(e=>{c(t=>{let i=Ke(t,r,n);return Ke(typeof e==`function`?e(i):e,r,n)})},[c,r,n]);(0,P.useEffect)(()=>{s!==l&&c(l)},[s,l,c]);let p=(0,P.useRef)(e);(0,P.useEffect)(()=>{p.current=e},[e]);let m=(0,P.useRef)(!1);(0,P.useEffect)(()=>{m.current=a!==null},[a]);let h=(0,P.useRef)(null);(0,P.useEffect)(()=>{if(!e){o(null);return}let t=new AbortController;return o(null),fetch(e,{signal:t.signal}).then(e=>{if(!e.ok)throw Error(`HTTP ${e.status}`);return e.json()}).then(e=>{let t=qe(e);t&&(o(t),f(`graph`))}).catch(()=>{}),()=>{t.abort()}},[e]);let g=(0,P.useCallback)(()=>{let e=p.current;if(!e)return;h.current?.abort();let n=new AbortController;h.current=n,d(!0),fetch(e,{signal:n.signal}).then(e=>{if(!e.ok)throw Error(`HTTP ${e.status}`);return e.json()}).then(e=>{let t=qe(e),n=t!==null&&!m.current;o(t),n&&f(`graph`),d(!1)}).catch(e=>{e.name!==`AbortError`&&(t(`Graph refresh failed: ${e.message}`,`error`),d(!1))})},[]);return(0,P.useEffect)(()=>()=>{h.current?.abort()},[]),{graphData:a,setGraphData:o,rightTab:l,setRightTab:f,isRefreshing:u,refetchGraph:g}}function Ye({bus:e,hasGraph:t,connected:n,refetchGraph:r,clearGraph:i,addToast:a}){let o=(0,P.useRef)(null),s=(0,P.useRef)(t),c=(0,P.useRef)(n),l=(0,P.useRef)(r);(0,P.useEffect)(()=>{s.current=t},[t]),(0,P.useEffect)(()=>{c.current=n},[n]),(0,P.useEffect)(()=>{l.current=r},[r]),(0,P.useEffect)(()=>{!n&&o.current!==null&&(clearTimeout(o.current),o.current=null)},[n]),(0,P.useEffect)(()=>e.on(`graph.mutation`,e=>{if(c.current){if(e.mutationType===`import-graph`){o.current!==null&&(clearTimeout(o.current),o.current=null),l.current(),a(`Graph imported — refreshing view…`,`info`);return}o.current!==null&&clearTimeout(o.current),o.current=setTimeout(()=>{o.current=null,c.current&&(l.current(),a(s.current?`Graph updated — refreshing…`:`Graph updated — opening Graph tab…`,`info`))},300)}}),[e,a]),(0,P.useEffect)(()=>e.on(`session.reset`,()=>{o.current!==null&&(clearTimeout(o.current),o.current=null),i()}),[e,i]),(0,P.useEffect)(()=>()=>{o.current!==null&&clearTimeout(o.current)},[])}function Xe(e){return e.replace(/^help\s*/i,``).trim().toLowerCase()}function Ze(e){let t=e.trim(),n=t.toLowerCase();if(n===`help`||n.startsWith(`help `))return Xe(t);let[r,i,a,...o]=n.split(/\s+/);return r===`describe`&&i===`skill`&&a!==void 0&&o.length===0?a.replace(/\./g,`-`):null}var Qe=`Connect two nodes
-----------------
Create a directional connection from one node to another with a descriptive
relation label.

Syntax
------
\`\`\`
connect {node-A} to {node-B} with {relation}
\`\`\`

Example
-------
\`\`\`
connect root to fetcher with fetch
\`\`\`

Notes
-----
- Connections are directional: 'connect a to b' is different from
  'connect b to a'.
- The relation is a free-form descriptive label (e.g. done, fetch, provider);
  it is not interpreted for skill routing. For data-entity nodes, meaningful
  relation names capture enterprise knowledge.
- Multiple outgoing connections from one node fork traversal into parallel
  branches, one per connection. Synchronize them with a graph.join node
  (see 'help graph-join').
- Every node must connect to at least one other node: a graph with orphan
  nodes cannot be exported for deployment (see 'help export'). Wire config
  nodes (Dictionary, Provider) and data entities under a graph.island node
  so no node is left unconnected (see 'help graph-island').
`,$e=`Create a node
-------------
Add a node to the current graph model. This is a multi-line command: enter
all lines as one block.

Syntax
------
\`\`\`
create node {name}
with type {type}
with properties
{key1}={value1}
{key2}={value2}
\`\`\`

Example
-------
\`\`\`
create node root
with type Root
with properties
name=helloworld
purpose=Demo graph
\`\`\`

Notes
-----
- Node names are matched case-insensitively; use lowercase letters, digits
  and hyphen. The names 'root' and 'end' are reserved: the root node must be
  named 'root' and the end node must be named 'end'.
- Types are descriptive labels, conventionally Capitalized (e.g. Root, End,
  Provider, Dictionary, Fetcher, Island). The type and properties are used
  and validated by the node's skill, if any.
- A node has zero or one skill, set with skill={route}.
- 'with properties' and the key lines are optional. Property values act as
  defaults for the instance model.
- A property key may be composite, using the dot-bracket format; a
  key[]=entry line appends one entry to the list "key" (repeat per entry).
  Values may use the Event Script constant syntax, e.g. text(hello),
  int(100), boolean(true).
- Wrap a multi-line value in triple single quotes (''').
- Best practice: give the root node a "name" property (the graph name) and a
  "purpose" property describing the use case as a one-liner.
`,et=`Data Dictionary
---------------
The data-dictionary method separates WHAT data to get from WHERE it comes
from. It uses three kinds of nodes:

1. Dictionary - defines one data attribute (or set of attributes)
   retrievable from a provider
2. Provider - defines the HTTP endpoint that supplies it
3. Fetcher - a node with skill=graph.api.fetcher that names dictionaries and
   makes the call(s) at run time (see 'help graph-api-fetcher')

Dictionary and Provider are CONFIGURATION nodes: they never execute and are
referenced by name (dictionary[]=..., provider=...). Do not leave them
floating - wire them into the knowledge layer under a graph.island node so
the graph carries its own entity-relationship diagram:
root -[contains]-> island -[data]-> dictionary -[provider]-> provider.
See 'help graph-island'.

Dictionary node
---------------
Defines one data attribute retrievable through a Provider.

\`\`\`
create node {name}
with type Dictionary
with properties
purpose={description}
provider={provider-node-name}
input[]={parameter}
input[]={parameter}:{default}
output[]=response.{path} -> result.{key}
\`\`\`

- input[] entries are BARE parameter names, not source -> target mappings
  (the one exception to the mapping rule). An optional :{default} suffix
  supplies a default value, e.g. input[]=detail:true - that is the ONLY
  meaning of ':' here.
- output[] maps the provider's raw HTTP response body (the response.*
  namespace) into the result set (result.{key}) that the fetcher exposes.
  The source path may be a leaf OR an interior node - an interior path maps
  the WHOLE subtree: response.profile.name -> result.name extracts one
  field, while response.profile -> result.profile captures the entire
  profile object and response.accounts -> result.account_numbers an entire
  array.

Example:

\`\`\`
create node person-profile
with type Dictionary
with properties
purpose=full profile record of a person
provider=mdm-profile
input[]=person_id
input[]=detail:true
output[]=response.profile.name -> result.name
output[]=response.profile.address -> result.address
\`\`\`

Provider node
-------------
Defines the HTTP call - the communication contract with the target system.

\`\`\`
create node {name}
with type Provider
with properties
purpose={description}
url={target url}
method={GET | POST | PUT | PATCH | DELETE | HEAD}
feature[]={feature flag}
input[]={source} -> {target}
\`\`\`

- The url may embed {name} path placeholders - each one is filled by an
  input[] line targeting path_parameter.{name}. Standard
  \${config.key:default} substitution also applies to the url.
- input[] sources: a constant (e.g. text(application/json)), a Dictionary
  parameter name (bare), or a state-machine value (model.*). Targets:
  header.{name}, query.{name}, path_parameter.{name}, body.{key} - or the
  whole "body" (e.g. to send a string or an array as the request body).
- feature[] entries declare capabilities the calling fetcher must support
  (e.g. an auth mechanism). Built-ins: log-request-headers and
  log-response-headers - the fetcher logs request/response headers into the
  "header" section of its properties. graph.api.fetcher prints a warning
  for a feature it does not support (a custom fetcher may enforce it).

GET example - a URL path placeholder filled from a dictionary parameter,
plus a JSON accept header:

\`\`\`
create node mdm-profile
with type Provider
with properties
purpose=MDM profile endpoint
url=http://127.0.0.1:\${rest.server.port:8080}/api/mdm/profile/{id}
method=GET
input[]=text(application/json) -> header.accept
input[]=person_id -> path_parameter.id
\`\`\`

POST example - body.{key} targets build the JSON request body; set the
content-type header (no URL placeholder - the parameters travel in the
body):

\`\`\`
create node account-api
with type Provider
with properties
purpose=account management endpoint
url=http://127.0.0.1:\${rest.server.port:8080}/api/account/details
method=POST
input[]=text(application/json) -> header.accept
input[]=text(application/json) -> header.content-type
input[]=person_id -> body.person_id
input[]=account_id -> body.account_id
\`\`\`

Putting it together
-------------------
The fetcher names the dictionary; the dictionary names the provider:

\`\`\`
create node fetcher
with type Fetcher
with properties
skill=graph.api.fetcher
dictionary[]=person-profile
input[]=input.body.person_id -> person_id
output[]=result.name -> output.body.name
output[]=result.address -> output.body.address
\`\`\`

The fetcher's input[] targets must match the dictionary parameter names
exactly, or execution fails. Full fetcher semantics (iterative fetching,
failure routing, deduplication): 'help graph-api-fetcher'.

Notes
-----
- Several Dictionary nodes may share one Provider - e.g. a provider returns
  a complex structure and each dictionary extracts different attributes.
  Identical calls (same provider + same input values) are deduplicated into
  a single HTTP request within a graph instance; only successful responses
  are cached.
- Dictionary and Provider nodes hold configuration only; the result set is
  stored on the FETCHER node ({fetcher}.result), not on the dictionary.
- Wire every dictionary and provider under the island knowledge layer
  (connect island to {dictionary} with data, connect {dictionary} to
  {provider} with provider) - leave no node unconnected.
`,tt=`Delete a node, a connection or the fetch cache
----------------------------------------------
Remove a node or the connections between two nodes from the current graph
model, or clear the API-fetcher response cache of the current graph instance.

Syntax
------
\`\`\`
delete node {name}
delete connection {node-A} and {node-B}
delete cache
\`\`\`

Example
-------
\`\`\`
delete node fetcher
delete connection root and fetcher
\`\`\`

Notes
-----
- Deleting a node also removes every connection touching it.
- 'delete connection' removes the connections between the two nodes in both
  directions, if any.
- 'delete cache' requires a graph instance (see 'help instantiate'). It
  clears the cache of successful API-fetcher responses, so the next
  identical call makes a real HTTP request instead of reusing a cached
  response.
- 'clear' is an alias of 'delete' (e.g. 'clear cache').
`,nt=`Describe graph, node, connection or skill
-----------------------------------------
Print the structure of the current graph model, the detail of a node or a
connection, or the documentation of a skill.

Syntax
------
\`\`\`
describe graph
describe graph {graph-id}
describe node {name}
describe connection {node-A} and {node-B}
describe skill {skill.route.name}
\`\`\`

- 'describe graph' (no id) describes the CURRENT DRAFT of this session.
- 'describe graph {graph-id}' (discovery, read-only) shows a DEPLOYED
  model's contract view: its purpose, node/connection counts, and the
  input.*/output.* data surface derived from the model's own mappings -
  everything needed to wire an extension= delegation without trial
  execution. Find the available ids with 'list graphs'.

Example
-------
\`\`\`
describe node fetcher
describe skill graph.api.fetcher
\`\`\`

Notes
-----
- 'describe graph' shows the structure of the current draft graph model.
- 'describe node' prints a node's type and properties.
- 'describe connection' reports the connections between the two nodes in
  either direction, or that they are not connected.
- 'describe skill' prints the shipped documentation of a skill by its route
  name - the same content as the hyphenated help topic (e.g.
  'help graph-api-fetcher').
`,rt=`Edit a node
-----------
A convenience command: prints an existing node as a complete 'update node'
command so you can copy it, edit the text, and submit the update.

Syntax
------
\`\`\`
edit node {name}
\`\`\`

Example
-------
\`\`\`
edit node demo-node
\`\`\`

Sample output
-------------
\`\`\`
update node demo-node
with type Demo
with properties
hello=world
test='''
this is a sample multi-line value
line two
line three
'''
good=day
\`\`\`

Notes
-----
- The printed command carries the node's current type and all properties,
  flattened to one key per line; list properties print one key[]=entry line
  per element, in order.
- Multi-line values are wrapped in triple single quotes.
- Edit the printed text and submit it as-is to apply the change (see
  'help update'). The node must exist, or the command reports an error.
`,it=`Execute a single node
---------------------
Run one node's skill in isolation. Graph traversal is paused, so you can
functionally verify a node without walking the whole graph.

Syntax
------
\`\`\`
execute node {name}
execute {name}
\`\`\`

Example
-------
\`\`\`
execute fetcher
\`\`\`

Notes
-----
- Requires a graph instance (see 'help instantiate').
- The node must have a 'skill' property with exactly one skill route, and
  that route must exist at runtime.
- The node reads from and writes to the instance's state machine exactly as
  it would during a run; use 'inspect' to check the outcome (see
  'help inspect').
- On success the console reports the execution time and the node's exit
  path; the node is marked as seen (see 'help seen').
`,at=`Export a graph model
--------------------
Write the current graph model as a JSON file for deployment or later
re-import.

Syntax
------
\`\`\`
export graph as {name}
\`\`\`

Example
-------
\`\`\`
export graph as helloworld
\`\`\`

Notes
-----
- The name uses letters, digits and hyphen; do not add a ".json" extension.
- The file is written to the Playground temp folder (configuration key
  location.graph.temp, default /tmp/graph).
- The export sets name={name} on the root node. If the root node's "name"
  property differs from {name} and the target file already exists, the
  export is refused - update the root node's name to overwrite the existing
  model. If no root node exists, one is created automatically.
- Export fails when the graph has orphan nodes: every node must connect to
  at least one other node (see 'help connect').
- The reply includes "Described in /api/graph/model/{name}/{token}", a
  read-only HTTP view of the exported model.
- In the Playground UI, the Download button beside Copy (Graph and Raw views)
  saves the model to your computer as {name}.json after asking for the graph
  id; the root node's name in the file becomes that id, as this command does.
  Browsers with a "save as" dialog let you pick the folder; others save to
  the download folder. Drop the file on the canvas, or use "Import Graph", to
  load it into a session again.
`,ot=`Skill: Graph API Fetcher
------------------------
Calls an external HTTP API declaratively. The node never holds a URL itself:
it names one or more Dictionary nodes (data attributes), and each Dictionary
names the Provider node (endpoint definition) that supplies it. When
traversal reaches the node, the fetcher resolves the provider through the
dictionary, makes the call(s), and collects the result set into the node's
"result" property.

Authoring the Dictionary and Provider configuration nodes is covered in
'help data-dictionary' - read that first.

Route name
----------
"graph.api.fetcher"

Properties
----------
\`\`\`
skill=graph.api.fetcher
dictionary[]={dictionary-node-name}
input[]={source} -> {dictionary-parameter}
output[]={source} -> {target}
\`\`\`

- dictionary[] (required) - one or more Dictionary node names configured in
  the same graph model. This is the only hard-required property.
- input[] - required whenever the dictionaries declare parameters (the usual
  case). Each entry's TARGET must match a dictionary parameter name exactly,
  or execution fails.
- output[] (optional) - maps the result set onward (e.g. to output.* or
  model.*). Optional because the result set always lands at {node}.result,
  where a later data mapper can pick it up.

Optional:

\`\`\`
for_each[]={array-source} -> model.{var}   (iterative fetching - see below)
concurrency={1-30}                         (parallel fan-out, default 3)
exception={error-handler-node}             (jump on failure instead of abort)
\`\`\`

Result set
----------
On success the result set - the values the Dictionary's output[] mappings
produced as result.{key} - is stored at {node}.result. In this node's own
output[] mappings, result.{key} reads from that set; later nodes read
{node}.result.{key}.

Example
-------
\`\`\`
create node fetcher
with type Fetcher
with properties
skill=graph.api.fetcher
dictionary[]=person-profile
input[]=input.body.person_id -> person_id
output[]=result.name -> output.body.name
output[]=result.address -> output.body.address
\`\`\`

Iterative fetching (for_each)
-----------------------------
A fetcher can execute once per element of a runtime array - the mechanism
for "fetch details for each item in a list obtained from a previous call":

\`\`\`
create node accounts-fetcher
with type Fetcher
with properties
skill=graph.api.fetcher
dictionary[]=account-detail
for_each[]=profile-fetcher.result.accounts -> model.account_id
concurrency=3
input[]=input.body.person_id -> person_id
input[]=model.account_id -> account_id
output[]=result.detail -> model.account_details
\`\`\`

- The for_each source MUST resolve to a list - typically a prior fetcher's
  result ({fetcher}.result.{key}) or a model.* array. Multiple for_each[]
  lines iterate multiple parameters in lock-step.
- Wire the current element into each call with an ordinary input mapping:
  input[]=model.{var} -> {dictionary-parameter}. Non-iterated inputs (like
  person_id above) pass unchanged to every call.
- concurrency bounds the parallel fan-out (1-30, default 3); calls run in
  batches of that size to avoid overwhelming the target service.
- Aggregation is GUARANTEED and ordered: each iteration's result.{key}
  values are appended into a single array on this node's result set - after
  N iterations, result.detail above is an array of N - and the aggregated
  array preserves the source list's order regardless of concurrency.

Failure routing (exception)
---------------------------
On a failed call (HTTP status >= 400):

- {node}.status and {node}.error are set (the engine's error record; {node}.stack is
  added when the failure carries a stack trace)
- the output[] mappings are SKIPPED
- with exception={handler-node}, traversal JUMPS to the handler; without
  it, the run ABORTS and the error is returned to the caller.

When traversal jumps to the handler, the engine also stages a generic exception context that
does not name the failing node:

- error.source  - the failing node's alias
- error.code    - the status code
- error.message - the error message
- error.stack   - the stack trace, when the failure carries one

so ONE handler node can serve the "exception" route of every node in the graph. A generic
handler reads the context in its data mapping without naming any failing node:

\`\`\`
mapping[]=error.source -> output.body.failed_at
mapping[]=error.code -> output.body.status
mapping[]=error.message -> output.body.message
\`\`\`

Anchor a shared handler from an island (root -> island -> handler): the handler is reached by
jumping, so the island keeps it non-orphan while plain traversal stops at the island. A handler
node may also connect onward to more nodes for sophisticated recovery (e.g. a graph.task
invoking a composable function). Note that a node is visited at most once per run unless RESET,
so if two parallel branches fail together, only the first jump enters a shared handler. The
alias 'error' is reserved for this namespace - probe it in a dry-run session with
"inspect error". When the failing node is retried successfully, the context resolves to
code=200 with the source kept and the failure details removed - the source match ensures a
parallel node's success never clears a different node's outstanding failure.

A retry handler is typically a graph.math decision node that inspects the
fetcher's status/error, counts attempts, and retries with a bound:

\`\`\`
create node error-handler
with type Decision
with properties
skill=graph.math
statement[]=RESET: {error.source}, error-handler
statement[]=MAPPING: f:defaultValue(model.attempts, int(0)) -> model.attempts
statement[]=MAPPING: f:add(model.attempts, int(1)) -> model.attempts
statement[]='''
IF: {model.attempts} >= 3
THEN: recovery-node
ELSE: next
'''
statement[]=NEXT: {error.source}
statement[]=DELAY: 50
\`\`\`

The handler is fully GENERIC: every statement command resolves {dynamic variables}, so
RESET: {error.source} and NEXT: {error.source} retry whichever node routed here - one handler
serves every fetcher and task in the graph. See tutorial 12 for the full walkthrough.

RESET comes first among the action statements so it runs on every path (a
taken IF jump ends the list) - the attempt counters live in the "model"
namespace, which RESET never touches. If the handler also carries a defensive
check on the failed node's status, that check must come BEFORE the RESET (it
reads state the reset wipes).

Wire the handler back explicitly (connect error-handler to fetcher with
retry) - no node left unconnected. See 'help graph-math' for the statement
grammar and the engine's loop guard.

Notes
-----
- One Provider call is exactly one HTTP request - redirects are never
  followed. A 3xx answer is a non-failure: its status and body are captured
  and traversal proceeds (only >= 400 triggers failure routing). Point the
  Provider url at the redirect target to land on it.
- {node}.status always carries the HTTP status of the fetch, success
  included (a 200 or a 301 is readable there, not just failures). The
  response.* namespace in a Dictionary output[] addresses the BODY only;
  the bare root (response -> result.page) captures a whole non-JSON body
  such as an HTML page.
- Deduplication: identical requests (same provider + same input values)
  within one graph instance are deduplicated into a single HTTP call. Only
  SUCCESSFUL responses are cached - a failed call is never cached, so a
  retry after RESET makes a real call, while an identical successful call
  reuses the cached response.
- Provider feature[] flags declare capabilities this fetcher must support.
  Built-ins: log-request-headers and log-response-headers - the fetcher
  logs request/response headers into the "header" section of its
  properties. An unsupported feature produces a warning (a custom fetcher
  may enforce it).
- Keep chains minimalist: fetchers can be chained to make multiple API
  calls, but an overly complex chain means slow performance. Take only the
  minimal set of data your application requires - don't abuse the
  flexibility of the API fetcher.
- Wire the Dictionary and Provider nodes into the island knowledge layer so
  no node is left unconnected - see 'help graph-island'.
`,st=`Skill: Graph Data Mapper
------------------------
Copies and transforms data between state-machine namespaces. Each mapping[]
entry moves one value from a source to a target when the node executes. This
is the workhorse skill for shaping inputs, staging intermediate values in
model.*, and assembling the response in output.body.

Route name
----------
"graph.data.mapper"

Properties
----------
\`\`\`
skill=graph.data.mapper
mapping[]={source} -> {target}
\`\`\`

- mapping[] (required) - one entry per line; entries execute in order,
  so a later entry may read an earlier entry's target (the chain idiom:
  ingest -> transform -> publish inside one mapper).

Sources: input.body / input.header, model.*, a node name (its properties),
{node}.result, a constant, an f:plugin(...) call, or a $. JSONPath
expression. Targets: output.body / output.header, model.*, or a node name.

Example
-------
\`\`\`
create node shape-response
with type Mapper
with properties
skill=graph.data.mapper
mapping[]=input.body.hr_id -> employee.id
mapping[]=fetch-one.result.profile -> output.body.profile[0]
mapping[]=fetch-two.result.profile -> output.body.profile[1]
mapping[]=f:now(text(local)) -> output.body.timestamp
\`\`\`

Decision table lookup
---------------------
A data mapper is also the natural decision node for a static decision table held on a skill-less node
(see "describe skill graph.task" for the table node). The "lookup" simple plugin returns the name of
the first rule whose list contains the value (compared as text, case-insensitively), or the optional
third argument on a miss (null when it is omitted):

\`\`\`
create node select-rule
with type Decision
with properties
skill=graph.data.mapper
mapping[]=f:lookup(state-rules, input.body.state, text(unknown)) -> output.body.rule
\`\`\`

The table's "keys" field lists the rule names in priority order and each rule field lists its values;
each may be a list or a JSON array written as text (keys=[ "a", "b" ]), and the table itself may be
JSON text. One table replaces a ladder of IF-THEN-ELSE and the product owner certifies it on the graph.

Dynamic keys
------------
A {namespace.key} reference inside a source resolves before the source is read, and its value is
inserted verbatim, never quoted. It composes a key segment, a list index, or text in a constant or a
plugin argument:

\`\`\`
mapping[]=census-2020.{model.state} -> model.population
mapping[]=input.body.items[{model.i}] -> output.body.item
mapping[]=text(Hello {input.body.name}!) -> output.body.greeting
\`\`\`

census-2020.{model.state} reads the property that model.state names: a keyed table (one KEY=value
line per key) answers a value per key, where f:lookup answers which rule lists a value.

- The reference may read any namespace (Event Script accepts model.* only), but only on the source
  side: a target is a literal path.
- An unresolved reference renders the text null, so a composed key misses and a constant reads
  "null". A composed key is case-sensitive, unlike f:lookup.
- Nothing is quoted, whatever a text constant contains ("!", "<", ">", "=="). A JSONPath filter is
  the one exception: a text value is quoted there so it reads as a string literal in the query.
- Keep a fixed prefix before a request value (census-2020.{input.body.state}) so the request
  chooses only within that table. A table value is text, and f:long(null) is -1, so refuse an
  unknown key before converting.

Constants
---------
A constant is valid wherever a source is. This is the full set:

- text(hello world) - string, verbatim (no quoting needed)
- int(100) / long(10000000000) - integer (non-numeric input yields -1; a
  decimal part is dropped)
- float(1.5) / double(1.5) - floating-point number
- boolean(true) - true only for case-insensitive "true"; anything else false
- map(k1=v1, k2=v2) - inline map literal (values are strings)
- map(config.key) - the value of an application-configuration key
- file(text:/tmp/f.txt) / file(json:...) / file(binary:...) - file content
  as text / parsed JSON / bytes
- classpath(text:/data/f.txt) - like file(), resolved against the app's
  resource roots

Beyond constants, two non-constant source forms are valid:

- f:plugin(args...) - a simple-plugin call, e.g. f:uuid(),
  f:now(text(local)), f:concat(model.a, text(!)), f:add(model.n, int(1)),
  f:ternary(...), f:defaultValue(input.body.flag, boolean(false)),
  f:removeKey(model.list, text(key)), f:listOfMap(...).
- $.  - a JSONPath expression over the state machine. Prefer plain
  dot-bracket keys; use JSONPath only when the query needs it.

Notes
-----
- Composite keys use dot-bracket form on both sides. A numeric index in a
  target creates/sets that list slot (profile[0], profile[1]) - the idiom
  for assembling a JSON list deterministically, e.g. after a fork/join. An
  empty index "[]" appends one element to the end of the list (and creates
  the list with that first element when it does not yet exist).
- An interior (non-leaf) source path maps the ENTIRE subtree, not just
  scalars - fetch-one.result.profile above carries the whole profile object.
- A NULL source (a missing key, or a plugin returning null) CLEARS a "model."
  target (removed; set to null when the source key exists or the target is
  indexed such as model.list[1]) and leaves any other target untouched - the
  same rule as Event Script. Defaults for a model variable come from the
  source side: f:defaultValue(input.body.flag, boolean(false)) -> model.flag,
  or a plugin default such as f:lookup(table, value, text(unknown)) - never
  default-then-overlay, which the null overlay would remove.
- The legacy colon-type suffix ("simple type matching") is deprecated - use
  the f:plugin forms instead.
- Inside a graph.math node, MAPPING: statements use exactly this syntax; see
  'help graph-math'.
`,ct=`Skill: Graph Extension
----------------------
Delegates to another graph model (a sub-graph) or to an Event Script flow,
so larger capabilities compose from smaller ones. The node passes named
inputs to the target, and the target's response body becomes this node's
result. This is the seam between the knowledge-graph layer and the Event
Script layer beneath it.

The delegated graph or flow inherits the caller's business correlation ID (model.cid), the same
way an Event Script sub-flow does. A delegated subgraph that suspends therefore persists its
state under the shared business correlation ID scoped by its own graph ID - re-invoking with the
same correlation ID resumes it. This makes a parent graph a natural orchestrator of independently
resumable subgraph paths (see the workflow-suspension guide's orchestrator pattern).

Route name
----------
"graph.extension"

Properties
----------
\`\`\`
skill=graph.extension
extension={graph-id}           (a deployed sub-graph ...)
extension=flow://{flow-id}     (... or an Event Script flow)
input[]={source} -> {key}
output[]={source} -> {target}
\`\`\`

- extension (required) - the target. A graph id resolves among DEPLOYED
  graph models only (compiled at startup from the app's resources/graph
  folder - the same ids callable at POST /api/graph/{graph-id}). A session
  draft is NOT addressable: export and deploy it first. A missing id fails
  the node fast at run time. A flow target takes the flow:// prefix, e.g.
  extension=flow://hello-world.
- input[] (required) - each entry's TARGET is a bare key that becomes the
  target's input.body.{key}. There is NO whole-body "*" target on this
  skill - map named keys (the "*" merge idiom is graph.task-only; see
  'help graph-task').
- output[] (optional) - maps the result onward; the result always lands at
  {node}.result regardless.

Optional:

\`\`\`
for_each[]={array-source} -> model.{var}   (iterate over a runtime list)
concurrency={1-30}                         (parallel fan-out, default 3)
exception={error-handler-node}             (jump on failure instead of abort)
\`\`\`

Result set
----------
This node's result namespace IS the target's output.body:

- bare "result" in an output[] mapping is the whole response body
- result.{key} is a field of it

The same contract applies to both target kinds: the named input keys feed
the sub-graph's or flow's input.body, and result.* is its output.body.

Example
-------
\`\`\`
create node performance-evaluator
with type Extension
with properties
skill=graph.extension
extension=evaluate-sales-performance
input[]=input.body.department_id -> id
output[]=result.sales_performance -> output.body.sales_performance
\`\`\`

Here input.body.department_id feeds the sub-graph's input.body.id, and the
sub-graph's output.body.sales_performance comes back as
result.sales_performance.

Notes
-----
- Failure routing: on failure, {node}.status and {node}.error are set and
  the output[] mappings are skipped. With exception={handler-node},
  traversal jumps to the handler instead of aborting; without it, the run
  aborts. The jump also stages the generic exception context
  (error.source/code/message and error.stack when available) so ONE
  island-anchored handler can serve every node - error.source is the
  extension node in THIS graph; failures inside the delegated subgraph or
  flow route to that graph's own handlers. The bounded-retry pattern and
  the full error-context contract are shown under 'help graph-api-fetcher'.
- for_each[]={array-source} -> model.{var} invokes the target once per
  element of a runtime list, with bounded parallel fan-out (concurrency
  1-30, default 3). The shared iteration rules are under
  'help graph-api-fetcher'.
- Use graph.extension for multi-step orchestration; use graph.task for a
  single composable-function call.
`,lt=`Skill: Graph Island
-------------------
Marks an isolated node. A node with this skill always returns ".sink", so
graph traversal never continues through it. That isolation is the point: the
island anchors the graph's knowledge layer. Dictionary, Provider, data-entity,
and reusable Module nodes hang off the island, turning the graph into its own
entity-relationship diagram - living documentation of the enterprise knowledge
behind the execution path.

Route name
----------
"graph.island"

Properties
----------
\`\`\`
skill=graph.island
\`\`\`

No other properties are required or accepted.

Example
-------
\`\`\`
create node dictionary
with type Island
with properties
skill=graph.island
\`\`\`

Wire the knowledge layer under it:

\`\`\`
connect root to dictionary with contains
connect dictionary to person-profile with data
connect dictionary to account-detail with data
connect person-profile to mdm-profile with provider
connect account-detail to account-api with provider
\`\`\`

Notes
-----
- Required convention: leave no node unconnected. Whenever the graph has
  off-path nodes - Dictionary/Provider configuration, data-entity, or
  reusable Module nodes - wire every one of them into the island structure:
  root -[contains]-> island -[data]-> dictionary -[provider]-> provider,
  and island -[module]-> module for reusable graph.math modules.
- Encouraged even for graphs with no off-path nodes: data-entity nodes that
  document the domain model (entities, fields, which fields are
  internal-only) make even a small graph discoverable enterprise knowledge.
- Relation labels are free-form and descriptive; "contains", "data",
  "provider" and "module" are the shipped conventions - choose names that
  capture the real-world relationship.
- Traversal is unaffected: the island sinks, so the run log shows a single
  "Executed ... with skill graph.island" line and the execution path never
  enters the knowledge layer.
- Reusable modules are documented under 'help graph-math'; the Dictionary and
  Provider configuration nodes under 'help data-dictionary'.
`,ut=`Skill: Graph Join
-----------------
A synchronization barrier for parallel branches. A node with this skill
returns "next" only when ALL upstream nodes connected to it have completed;
until then it returns ".sink" (the arriving path pauses). Use it to bring
forked branches back together before continuing.

Completion is success-only and current: a branch that failed into its
"exception=" route does not count while it retries, and a RESET node stops
counting until it re-executes successfully - so a retry loop feeding a join
holds the barrier instead of firing it prematurely. A chained upstream join
counts only once it actually FIRED (an evaluation that sank does not count),
so multi-stage joins compose safely.

Route name
----------
"graph.join"

Properties
----------
\`\`\`
skill=graph.join
\`\`\`

No other properties are required or accepted.

Example
-------
\`\`\`
create node join
with type Join
with properties
skill=graph.join
\`\`\`

Fork, then join:

\`\`\`
connect root to fetch-name with fetch
connect root to fetch-address with fetch
connect fetch-name to join with done
connect fetch-address to join with done
connect join to combine with proceed
\`\`\`

Notes
-----
- The fork side needs no special node: multiple outgoing connections from one
  node run their branches in parallel.
- A join is only meaningful with two or more upstream connections. Without a
  join, traversal simply proceeds as each branch completes.
- Data mapping is thread-safe (state-machine operations are serialized), but
  parallel branches must not write the SAME scalar key - the last writer
  wins, nondeterministically. Use disjoint keys (e.g. per-branch model.*
  variables), or append to a shared list with the race-free "[]" target form
  (element order then follows completion order). When the final order must
  be deterministic, assemble with numeric indices after the join, e.g.
  fetch-name.result.profile -> output.body.profile[0]. See
  'help graph-data-mapper'.
`,dt=`Skill: Graph JS (deprecated)
----------------------------
The graph.js skill evaluates inline JavaScript statements for computation and
decision-making. It is DEPRECATED: do not author new graph.js nodes. Use
graph.math for inline computation and IF/THEN/ELSE decisions - its expression
dialect covers the same ground without a script engine (see 'help graph-math') -
and graph.task to invoke a composable function for any logic an inline
expression cannot express (see 'help graph-task').

Status by engine
----------------
- Java engine: still registered so that existing graph models keep running;
  the module is scheduled for removal once field installations have migrated.
  Runtime JavaScript is an injection surface, and an equality comparison with a
  quoted string literal was measured silently evaluating to false - the reasons
  are recorded in the skills reference (graph.js section).
- Rust engine: never registered. A node with skill=graph.js fails at execution
  time with:

\`\`\`
Skill graph.js is retired for security reasons - use graph.math or graph.task instead
\`\`\`

Migrating a graph model
-----------------------
When importing an older graph model that contains graph.js nodes, replace
skill=graph.js with graph.math (compute/branch) or graph.task (custom logic)
before running it. The statement grammar has the same shape - COMPUTE, IF,
MAPPING, EXECUTE, RESET, NEXT, DELAY and for_each with BEGIN/END - so most
nodes migrate by changing the skill and rewriting each expression in the
graph.math dialect (a closed set of operators and functions, listed under
'help graph-math').

Reference for existing graph.js nodes (Java engine only)
--------------------------------------------------------
Route name: "graph.js"

Properties:

\`\`\`
skill=graph.js
statement[]=COMPUTE: variable -> JavaScript statement
statement[]=IF: if-then-else statement
statement[]=MAPPING: source -> target
statement[]=EXECUTE: another-node
statement[]=RESET: node-name
\`\`\`

Optional properties:

\`\`\`
for_each[]={map an array parameter for iterative statement execution}
statement[]=BEGIN
statement[]=END
statement[]=NEXT: {next-node-name}
statement[]=DELAY: {milliseconds}
\`\`\`

- Statements execute in order. A node with only MAPPING statements is rejected -
  use graph.data.mapper for mapping-only work.
- A COMPUTE result is stored in the node's "result" namespace; a later MAPPING
  statement can map it onward.
- An IF statement evaluates a boolean operation and may override the natural
  traversal order by jumping to a named node; when every statement resolves to
  "next", natural traversal is preserved.
- A node executes once per run (the run-once guard). RESET: clears a node's
  "seen" status and its result so conditional traversal can run it again;
  use it with care.
- for_each[] with BEGIN/END iterates a statement block over a runtime array,
  NEXT: jumps to a named node and DELAY: pauses before the next node - the same
  rules as graph.math. Every statement command resolves {dynamic variables}, so
  NEXT:/THEN:/ELSE: targets, RESET: entries and DELAY: values may each be a
  {namespace.key} reference (e.g. NEXT: {error.source} in a generic error
  handler).
- The skill is designed for a simple inline JavaScript statement using the
  standard JavaScript library; complex functions and variables are not
  supported.

COMPUTE statement:

\`\`\`
create node demo-js-runner
with properties
skill=graph.js
statement[]=COMPUTE: amount -> (1 - {input.body.discount}) * {book.price}
\`\`\`

The syntax {variable_name} resolves a value from the "input." or "model."
namespace or from a node's properties into the statement. A later statement
can use the result of a prior statement as its parameter.

IF statement - a multi-line command:

\`\`\`
statement[]='''
IF: (1 - {input.body.discount}) * {book.price} > 5000
THEN: high-price
ELSE: low-price
'''
\`\`\`

THEN: and ELSE: each name the node to jump to, or the keyword "next". When the
JavaScript statement does not return a boolean, the result is coerced: a
positive number is true and a negative number false; the text values "true",
"yes", "T" and "Y" are true and any other text is false; any other value is
converted to text first. (graph.math never coerces - a boolean is never a
number there.)

MAPPING statement - identical to the data mapper, so no curly braces:

\`\`\`
statement[]=MAPPING: input.body.hr_id -> employee.id
statement[]=MAPPING: input.body.join_date -> employee.join_date
\`\`\`

EXECUTE statement - runs another graph.js node's statements:

\`\`\`
statement[]=EXECUTE: js-3
\`\`\`

The "[]" suffix appends one statement per line to the node's statement list.
`,ft=`Skill: Graph Math
-----------------
Fast inline math and boolean evaluation for computation and decision-making.
A node with this skill runs an ordered list of statement[] lines. This is the
skill for inline compute and branching (graph.js is deprecated - see
'help graph-js'). For anything richer than the narrow expression dialect
described below, invoke a composable function instead (see 'help graph-task').

Route name
----------
"graph.math"

Properties
----------
\`\`\`
skill=graph.math
statement[]=COMPUTE: {var} -> {expression}
statement[]=DECIMAL: {var} -> {expression}   (exact decimal arithmetic - see below)
statement[]=CONDITION: {var} -> {boolean expression}
statement[]=IF: / THEN: / ELSE:              (multi-line - see below)
statement[]=MAPPING: {source} -> {target}
statement[]=EXECUTE: {node-name}
statement[]=RESET: {node-name}[, {node-name} ...]
\`\`\`

- statement[] (required) - at least one statement; statements run in order.

Optional:

\`\`\`
for_each[]={array-source} -> model.{var}     (iterate a statement block)
statement[]=BEGIN / statement[]=END          (delimit the for_each block)
statement[]=NEXT: {node-name}
statement[]=DELAY: {milliseconds}
\`\`\`

Statements
----------
- COMPUTE: {var} -> {expression} - evaluate the expression; the result is
  stored in THIS node's result namespace, readable as
  {this-node}.result.{var} or moved onward with a MAPPING statement.
- DECIMAL: {var} -> {expression} - the high-precision COMPUTE: the expression
  is evaluated with exact decimal arithmetic and the result is stored in the
  node's result namespace as a canonical decimal string (plain notation, the
  computed scale kept, a zero of any scale written "0"). COMPUTE still computes
  in binary floating point, and a graph that never says DECIMAL keeps its
  arithmetic; two rules reach it all the same: a string that is a canonical
  number compares as a number, and round() is half up, away from zero. See
  "DECIMAL statement" below.
- CONDITION: {var} -> {boolean expression} - the declared boolean statement:
  evaluated as a boolean whatever operators it carries (a bare {model.flag}
  included) and stored as a boolean in THIS node's result namespace; an IF
  may test it directly (IF: {this-node}.result.{var}). A COMPUTE stores a
  boolean only when its expression happens to carry a comparison or boolean
  operator - CONDITION says so in the statement.
- IF - a boolean decision that can redirect traversal (see below).
- MAPPING: {source} -> {target} - data mapping, identical to the data mapper
  (see 'help graph-data-mapper'). Do NOT wrap source/target in curly braces.
  A node with ONLY MAPPING statements is rejected - use graph.data.mapper.
- EXECUTE: {node-name} - run another graph.math node's statements inline, IN
  THE CALLING NODE'S CONTEXT: any COMPUTE results land on the INVOKING node
  ({invoker}.result.{var}); the executed module's own namespace stays empty.
  This is the module-reuse mechanism - author a formula once in an off-path
  Module node reading neutral model.* operands, and any node borrows it.
- RESET: {node-name}[, ...] - forget one or MORE nodes completely (the
  run-once guard, the completion mark, and the node state; comma/space
  list). A reset node stops satisfying a graph.join barrier until it
  re-executes successfully. Resetting a never-executed node is a safe
  no-op. Advanced - see Notes.

Expressions
-----------
{namespace.key} substitutes a value from input.*, model.*, or a node's
properties/result into a COMPUTE or IF expression, e.g.
{input.body.discount}, {book.price}, {model.x}. Substitution is robust to
hyphenated names - {unit-price} is the value of "unit-price", never parsed
as a subtraction - so use communicative hyphenated names freely.

The dialect is a NARROW JavaScript-like subset evaluated by the engine's own
parser, not a JavaScript runtime - a closed set. It accepts exactly the
following; an operator, function or constant not listed here is rejected by
name ("Unknown function: hypot"), never silently. COMPUTE yields a double, so
an integer result serializes as e.g. 8.0 (numerically exact).

\`\`\`
Literals   : numbers (42, 3.14, .5, 1e-5), strings ('text' or "text"), booleans (true, false)
Variables  : {namespace.key} substitution only - e.g. {input.body.qty}, {model.total}, {book.price};
             an unresolved selector fails by name before evaluation
Operators  : **  exponent, right-associative; a unary operand needs parentheses: -(2 ** 2), never -2 ** 2
             unary + - !          * / % (remainder)          + - (+ concatenates when either side is a string)
             < <= > >= (two numbers, or two strings compared lexically)
             == != (same type on both sides)          && || (short-circuit)          test ? a : b          ( )
Functions  : sin, cos, tan, asin, acos, atan, sqrt, abs, floor, ceil, round, log, log10, exp   (one argument)
             pow(x, y)          min(a, b, ...)          max(a, b, ...)          random()
             every function is also available as Math.name, e.g. Math.pow(2, 3)
Constants  : PI, E (also Math.PI, Math.E)
Not in the dialect: bitwise and shift operators (& | ^ ~ <<), assignment (=), user-defined variables
             and functions, arrays, objects, string methods - use a graph.task function instead
\`\`\`

Precedence, tightest first: ** > unary > * / % > + - > relational > equality > && > || > ?:

Numbers and booleans - each rule is enforced by a named failure, never a
silent value:
- A boolean is not a number. A boolean where arithmetic, a < or > comparison
  or a function argument needs a number fails naming the selector, e.g.
  "Boolean operand: model.flag (true) in '{model.flag} + 1' - a boolean is
  not a number; store a boolean with CONDITION or assert the type with
  f:validate". A JSON true in a numeric slot never computes as 1. Equality
  (==, !=) type-checks its two sides; a string that is a canonical number
  counts as a number, so '200' == 200, 200 == '200' and '200' == '200' are the
  same comparison and '9.5' < '10.25' compares 9.5 with 10.25.
- A misspelled or unsupported function fails by name ("Unknown function: mn").
- Arithmetic is IEEE double precision. An overflow to infinity, a division by
  zero and a NaN each fail naming the operator ("Arithmetic overflow in '*'
  (result Infinity)", "Division by zero or arithmetic overflow in '/'");
  integers beyond 2^53 lose precision; round() is half up, away from zero
  (round(-2.5) is -3), the same as f:round. Money that needs exact decimal
  arithmetic or a stated rounding mode belongs in a DECIMAL statement, the
  high-precision COMPUTE; COMPUTE stays floating point.

DECIMAL statement
-----------------
\`\`\`
statement[]=DECIMAL: fee -> {input.body.amount} * {input.body.rate}
statement[]=DECIMAL: rounded -> round({price.result.fee}, 2, HALF_UP)
\`\`\`

Numbers or strings - a conscious decision: a decimal may arrive as a string
("0.0375") or as a JSON number (0.0375), and both give the same answer. A JSON
number is a double, and DECIMAL converts it through the shortest decimal text
it prints as, at its minimal scale (5.0E-4 becomes 0.0005, 100.0 becomes 100).
That is exact over the text received, but a double that was already computed in
floating point is only as exact as that computation: COMPUTE: 1.005 * 100 is
100.49999999999999, and rounding that in a DECIMAL statement gives 100 where
the exact 100.5 gives 101; and a JSON number longer than a double holds was
rounded by the parser. So send money as strings and keep a COMPUTE result out
of a DECIMAL statement; to insist on strings, assert the type:
f:validate(input.body.rate, text(rate; String; required)). Whole numbers are
exact either way. The result is a string on purpose: graph.suspend saves the
state machine and graph.resume restores it, and a string is the same after as
before.

Arithmetic: + - * are exact; / never truncates (the exact quotient when it
terminates, otherwise 34 significant digits, half-even); % is the remainder;
** and pow(x, n) take a whole exponent from -999 to 999; abs, floor, ceil, min
and max are exact. Rounding is always explicit: round(x, scale, mode) with mode
HALF_UP, HALF_EVEN, HALF_DOWN, UP, DOWN, CEILING or FLOOR. What cannot be exact
is refused by name: sqrt, log, log10, exp, trigonometry, random(), PI and E -
keep that step in a COMPUTE or a graph.task function. A DECIMAL statement
computes a number; a comparison may appear only inside a ternary test. A COMPUTE
on a decimal string computes in binary floating point, so use DECIMAL for money.

A zero of any scale is stored as "0" (round(0.004, 2, HALF_UP), 1.50 - 1.50 and
0.00 all give "0"), and "0" + "1.50" is "1.50": the next scaled addend restores
the scale. The remainder (%) follows the sign of the dividend (-7 % 3 is "-1").
The decimal plugins (f:decimalAdd, f:decimalRound ...) work in a MAPPING
statement.

IF / THEN / ELSE
----------------
IF is the decision construct. It is a multi-line statement - enter it as one
statement[] value wrapped in triple single quotes. THEN: and ELSE: are both
REQUIRED, or the engine aborts the run.

\`\`\`
statement[]='''
IF: {input.body.a} >= {input.body.b}
THEN: ge-path
ELSE: lt-path
'''
\`\`\`

- THEN: / ELSE: each name the node to jump to, or the keyword "next".
- A taken node-jump ENDS the statement list immediately - later statements
  do not run. A branch resolving to "next" FALLS THROUGH: processing
  continues with the following statements, and natural traversal is
  preserved if nothing else redirects it. Order the list accordingly (e.g.
  an early-exit check first, retry logic after).

Traversal control
-----------------
- NEXT: {node-name} - unconditionally jump to a node BY NAME (a node name,
  not a connection label). Unlike a taken IF jump, NEXT: does not stop
  processing: the remaining statements still run, and the jump applies after
  the whole list completes (the last NEXT: wins).
- DELAY: {milliseconds} - pause after this node completes, before the walk
  continues to the next node. Paces retries; simulates a slow service.
- RESET enables retry loops. A node may reset ITSELF - the run-once mark is
  set before execution, so a self-reset survives and the node can run again.
  Placement rule: put RESET FIRST among the action statements - it then runs
  on every path (a later taken IF jump would skip it) and everything the node
  stores afterwards (such as DELAY's pending pause) survives the self-wipe.
  The one exception: keep RESET after any statement that reads state it would
  wipe - an IF on a just-wiped variable (e.g. {fetcher.status} after
  RESET: fetcher) aborts the run, so a defensive status check goes before it.

Dynamic variables in statement commands
---------------------------------------
Every statement command resolves {dynamic variables}, not only expressions. A NEXT: or THEN:/ELSE:
jump target, a RESET: list entry and a DELAY: value may each be a {namespace.key} reference
resolved at execution time. This is what makes a GENERIC error handler possible - it retries
whichever node routed to it without naming any node:

\`\`\`
statement[]=RESET: {error.source}, error-handler
statement[]=NEXT: {error.source}
statement[]=DELAY: {model.backoff}
\`\`\`

An unresolved variable renders "null": a RESET: entry is then a safe no-op, a DELAY: is skipped,
and a jump target fails the run loudly ("Next node 'null' does not exist") - correct for a jump,
so seed the variable before relying on it. A COMPUTE: or IF: expression over an unresolved variable
fails before evaluation and names it ("Unknown identifier: model.backoff (unresolved variable in
'{model.backoff} * 2')") - every unresolved variable when there are several ("Unknown identifier:
model.threshold or model.factor") - so the node that failed to set it can be found. See tutorial 12
for the full generic retry handler.

Iterating lists (for_each)
--------------------------
for_each[] turns part of the statement list into a loop. Each entry has the
mapping form {source} -> model.{var}; the right-hand side MUST be a model.*
key.

- A LIST-valued source becomes an iteration array: model.{var} is rebound to
  element i on each pass. Multiple list entries advance in LOCKSTEP (parallel
  arrays) and must all have the same length. At least one entry must resolve
  to a list, or the node aborts.
- A SCALAR source binds its model.{var} once, before the loop - even when
  the lists are empty.
- An UNRESOLVABLE source REMOVES the model.{var} key.

BEGIN and END split the statements into three blocks:

\`\`\`
statement[]=...       <- pre-block: runs ONCE, before the loop
statement[]=BEGIN
statement[]=...       <- each-block: runs once PER ELEMENT
statement[]=END
statement[]=...       <- post-block: runs ONCE, after the loop
\`\`\`

- Without BEGIN, the WHOLE statement list is the loop body - seed
  accumulators in a pre-block, or the seeding re-runs on every iteration.
- Iteration is strictly SEQUENTIAL, in list order, inside one node execution
  (a long list does not trip the loop guard). Contrast: the API fetcher's
  for_each fans HTTP calls out concurrently - see 'help graph-api-fetcher'.
- A taken IF jump BREAKS the loop: it ends the current iteration, skips the
  remaining elements and the post-block, and redirects traversal. An
  "ELSE: next" falls through within the iteration.
- Empty lists are fine: the each-block runs zero times; pre/post still run.
- COMPUTE yields doubles; the f:add family uses numeric promotion - inputs
  that are all whole numbers keep exact long arithmetic (including integer
  division), while any decimal argument promotes the whole computation to a
  double. So f:add composes directly with COMPUTE results; accumulate with
  either f:add or a pure-COMPUTE read-back, as below. Tame floating-point
  precision artifacts with f:round(value, int(2)) - half-up rounding on the
  number's decimal representation (1.005 rounds to 1.01 at 2 places).

\`\`\`
create node totaler
with type Loop
with properties
skill=graph.math
for_each[]=input.body.prices -> model.price
for_each[]=input.body.quantities -> model.qty
statement[]=MAPPING: int(0) -> model.total
statement[]=BEGIN
statement[]=COMPUTE: total -> {model.total} + {model.price} * {model.qty}
statement[]=MAPPING: totaler.result.total -> model.total
statement[]=END
statement[]=MAPPING: model.total -> output.body.total
\`\`\`

With prices=[10,20,30] and quantities=[7,8,9] the run yields total: 500.0 -
the pre-block seeds the accumulator once, each pass computes
total + price*qty and writes it back, and the post-block maps the final
value out. The plugin form is equivalent:
COMPUTE: line -> {model.price} * {model.qty} followed by
MAPPING: f:add(model.total, totaler.result.line) -> model.total.

Example
-------
\`\`\`
create node price-check
with type Decision
with properties
skill=graph.math
statement[]=COMPUTE: amount -> (1 - {input.body.discount}) * {book.price}
statement[]='''
IF: (1 - {input.body.discount}) * {book.price} > 5000
THEN: high-price
ELSE: low-price
'''
\`\`\`

Reusable module - author the formula once, borrow it anywhere:

\`\`\`
create node addition
with type Module
with properties
skill=graph.math
statement[]=COMPUTE: sum -> {model.a} + {model.b}
\`\`\`

\`\`\`
create node calculate
with type Compute
with properties
skill=graph.math
statement[]=MAPPING: input.body.a -> model.a
statement[]=MAPPING: input.body.b -> model.b
statement[]=EXECUTE: addition
statement[]=MAPPING: calculate.result.sum -> output.body.sum
\`\`\`

Note "calculate.result.sum", not "addition.result.sum" - the caller borrows
the logic, so the result belongs to the caller. Keep the module off the
execution path and hang it under the island knowledge layer
(island -[module]-> addition) - see 'help graph-island'.

Notes
-----
- A node executes ONCE per run (the run-once guard); a RESET statement is
  the only escape, for advanced re-execution. Use it with care.
- Loop guard: a node executed too frequently (default: more than 10 times
  per second) aborts the traversal - bound every retry loop and pace it
  with DELAY:.
- for_each[]={array-source} -> model.{var} iterates a statement block over a
  runtime array; BEGIN / END delimit the block to iterate (they are for_each
  delimiters, not IF braces) - see "Iterating lists" above. Without
  for_each[], BEGIN/END lines are accepted and ignored.
- The bounded-retry pattern (RESET the failing node and itself first, count
  attempts with f:defaultValue + f:add, exit at the bound via a taken IF
  jump, NEXT: back, DELAY: to pace) is shown under 'help graph-api-fetcher'.
`,pt=`Skill: Graph Resume
-------------------
When a graph run starts with the same business correlation ID as a previously suspended
transaction, the node with this skill restores the persisted workflow state and continues
traversal from the recorded suspension point - without re-executing it.

This skill is a superset of "graph.task": the "task" property names the pluggable store
function, but restoration is encapsulated by the skill, so the node needs no input or
output data mapping.

Place the resume node early in the traversal - conventionally named "resume" and connected
right after "root", or after nodes that perform setup and initialization. When the store
has a record for the business correlation ID (model.cid), the skill merges the persisted
model key-values into the state machine (the current run's reserved keys such as model.cid
and model.instance always win), restores the traversal bookkeeping so downstream join
barriers still see branches completed before suspension, and jumps past the suspension
point onto its normal forward path.

When there is no record - a fresh transaction, which is the normal first-run case, or an
expired record - traversal simply continues along the resume node's own forward path.

Either way, the skill records the outcome in "model.run" - "resume" when a record was
restored, "fresh" when there was none. The engine does not distinguish an absent record
from an expired one; with several checkpoints in one graph, no single fallback could be
right for all of them. Handling the fresh-or-expired condition is application logic:
gate the resume node's forward path with a graph.math IF-THEN-ELSE (on model.run or on
the request shape - see "help tutorial 14") to reject the request, advise the UI, or
jump to a recovery node.

The default store behavior consumes the record on retrieval, so a duplicate resume request
cannot execute the continuation twice.

Route name
----------
"graph.resume"

Setup
-----
To enable this skill, set "skill=graph.resume" as a property in a node.

The following parameter is required in the properties of the node:

1. task - the route name of the state-store function (e.g. "v1.redis.retrieve.model")

The store function receives headers "type=get" and a body of {"cid": "...", "graph": "..."}
(the running graph's ID scopes the lookup, so a resume only ever sees records written by its
own graph - parent and subgraphs are self-contained) and returns
the persisted record, or nothing (null or an empty map) when absent or expired.

Example
-------
create node resume
with type Resume
with properties
purpose=Restore workflow state from the external state store
skill=graph.resume
task=v1.redis.retrieve.model
`,mt=`Skill: Graph Suspend
--------------------
When a graph reaches the node with this skill, the workflow state of the graph instance is
persisted to an external state store and the graph run completes normally - the transaction
can resume later through the "graph.resume" skill using the same business correlation ID.

This skill is a superset of "graph.task": the "task" property names the pluggable store
function, but the persistence envelope is assembled by the skill itself, so the node needs
no input or output data mapping.

The node carrying this skill MUST be named "suspend" - a reserved alias like "root" and
"end". There is exactly one suspend node per graph, and two patterns reach it - named
after the node that pauses:

1. Checkpoint node - a working node with a DRAWN EDGE to the "suspend" node pauses when
   its skill completes normally: the walker redirects to "suspend" instead of following
   the node's continuation edge. The drawn edge is the declaration - no node property is
   needed. The node must also have at least one other edge (the continuation): a resumed
   run continues along it, and the node itself is never re-executed. A checkpoint never
   decides - reaching it IS the decision to pause. It is a complete working node: it
   executes its skill in full (input mapping, skill, output mapping) before pausing, so
   it may carry any non-routing skill (graph.data.mapper, graph.task, graph.api.fetcher,
   graph.extension), capture the actor's input into the model, and stage the caller's
   reply in output.* - only its exit changes.

2. Decision node - a decision (graph.math) pauses by returning "suspend" from its
   IF-THEN-ELSE. On resume the decision is RE-EXECUTED against the new request input,
   so it re-decides every time: an approval proceeds, a rejection terminates, and
   anything else returns "suspend" again - a wait loop with no extra nodes. A decision
   must NOT draw an edge to the suspend node (its drawn edges are outcome alternatives,
   and the gate rejects the shape); it may stage the caller's waiting reply in output.*
   before its IFs - the outcome paths overwrite it.

(The ADRs and compiler internals call these shapes "edge mode" and "jump mode".)

When the suspend node is reachable only by jumps, anchor it behind an island so the graph
has no orphan nodes: "root -> island -> suspend" - traversal stops at the island, so the
anchor edge is never walked. The suspend node cannot be an exception handler
(exception=suspend is rejected). The retired "suspend=true" property is accepted and
ignored for one deprecation window (the gate logs a WARN) - every valid earlier model
already draws its checkpoint edge, which now declares the same behavior.

A suspension point must be the sole active branch - do not suspend between a fan-out and
its join; suspend after the join instead. Anything a later step needs must be mapped into
the "model" namespace before the suspension point, because a node's transient "result"
properties do not survive suspension - the model is the workflow's durable memory.

Unless the graph staged its own output before suspension, the skill stages a default
response body so the caller of the suspended run receives a meaningful reply:

{
  "type": "suspended",
  "cid": "<business correlation ID>"
}

Route name
----------
"graph.suspend"

Setup
-----
To enable this skill, create a node named "suspend" with "skill=graph.suspend".

The following parameters are required in the properties of the node:

1. task - the route name of the state-store function (e.g. "v1.redis.persist.model")
2. ttl - the record's time-to-live using duration syntax, e.g. 20s, 5m, 2h, 2d

The store function receives headers "type=put" and a body of:

{
  "cid":   "<business correlation ID>",
  "graph": "<the graph that suspended - cid + graph form the retrieval key>",
  "node":  "<the suspension point - the node that routed here>",
  "ttl":   <seconds>,
  "model": { the model namespace minus the per-run reserved keys },
  "seen":  { traversal bookkeeping },
  "run":   { traversal bookkeeping }
}

The record is scoped by graph + cid (the Redis reference implementation keys it
"graph:{graph_id}:{cid}"), so the same business transaction may suspend independently in
each domain's graph and in each subgraph, and a resume only ever sees its own graph's
record. When the graph runs as one iteration of a parent's for_each fan-out, the iteration
index is appended as a third segment ("graph:{graph_id}:{cid}:{index}") so concurrent
iterations do not collide. The store must acknowledge with a 2xx reply before the graph completes - a failed
store call fails the node (the optional "exception" property routes it to a handler node).

Example
-------
create node suspend
with type Suspend
with properties
purpose=Persist workflow state to the external state store
skill=graph.suspend
task=v1.redis.persist.model
ttl=2d
`,ht=`Skill: Graph Task
-----------------
When a node is configured with this skill of "graph task", it will invoke a composable function
through its route name and collect the function's response into the "result" property of the node.
In case of exception, the "status" and "error" fields will be set to the node's properties and the
graph execution will stop unless an exception handler node is configured.

A composable function is a TypedLambdaFunction registered using the PreLoad annotation. This provides
a lightweight method to extend a knowledge graph's capability with a small piece of business logic,
without writing a new skill - more complex business logic should be delegated to a flow extension
or a subgraph using the "graph.extension" skill.

Execution will start when the GraphExecutor reaches the node containing this skill.

Route name
----------
"graph.task"

Setup
-----
To enable this skill for a node, set "skill=graph.task" as a property in a node.

The following parameters are required in the properties of the node:

1. task - the route name of the composable function to invoke
2. input - one or more data mapping entries as input to the composable function

The system uses the same syntax of Event Script for data mapping.

Properties
----------
\`\`\`
skill=graph.task
task=route.name.of.composable.function
input[]={mapping of key-values from input, model or another node to the function's request}
output[]={optional mapping of result set to one or more variables in the 'model.' or 'output.' namespace}
\`\`\`

Optional properties
-------------------
\`\`\`
for_each[]={map an array parameter for iterative function execution}
concurrency={controls parallel function calls for an "iterative task request". Default 3, max 30}
exception={error-handler-node-name}
\`\`\`

Input data mapping
------------------
source.composite.key -> target

The source (LHS) can use a key-value from the \`input.\` namespace, the \`model.\` namespace, another
node or a constant such as text(hello). The target (RHS) addresses the function's request:

1. \`*\` - the LHS value becomes the whole request body (same as Event Script). Data mapping entries
   are processed in order, so later entries can merge additional key-values into a request body
   that was seeded with \`*\`.
2. \`header.{name}\` - sets a request header of the function call
3. \`model.{key}\` - stages a variable in the graph's state machine instead of the request body, so
   that later entries can reference it as a **dynamic variable** (same as Event Script). e.g. after
   \`input.body.token -> model.token\`, the entry \`text(Bearer {model.token}) -> auth\` resolves the
   \`{model.token}\` reference. Engine-managed model metadata (model.cid, model.ttl, etc.) is
   immutable - a mapping that targets it is rejected.
4. any other composite key - a key-value in the request body

Example:
\`\`\`
input[]=input.body -> *
input[]=input.header.hello -> header.hello
input[]=input.body.amount -> amount
input[]=input.body.person_id -> model.person_id
input[]=text(/api/mdm/profile/{model.person_id}) -> url
\`\`\`

Static decision table
---------------------
A lookup table that changes with legislation rather than with each request belongs on a node, not
in the function. It is more readable - the product owner certifies the rules on the graph, in the
business vocabulary - and it replaces a ladder of IF-THEN-ELSE (a chain of graph.math decision nodes,
or conditionals inside a composable function) with one table that grows a row per rule instead of a
branch per rule. Do not hard-code the table in graph.math statements or in a function. Every node's
properties are copied into the state machine when the graph is instantiated, so a skill-less node's
alias is a mapping source and ONE input entry hands the whole table to a generic function:

\`\`\`
create node state-rules
with type DecisionTable
with properties
keys=[ "community-property", "separate-property" ]
community-property=[ "CA", "TX" ]
separate-property=[ "NY" ]
\`\`\`

\`\`\`
input[]=state-rules -> table
input[]=input.body.state -> key
\`\`\`

"keys" names the rules in priority order and each rule lists the values that select it. Every value
is a JSON array written as text, so the node reads as a table in the Playground and the function
reconstructs the lists (SimpleMapper in Java, serde_json in Rust) and reads the rule names from
"table.keys". For the common case no function is needed at all: a graph.data.mapper decision node
resolves the rule in one entry with the "lookup" simple plugin, its optional third argument being the
default for a miss - see "describe skill graph.data.mapper". Variations: "key[]=member" lines build a real list property,
and a nested table may be ONE JSON text property (a multi-line '''...''' value) that f:json parses
at mapping time: input[]=f:json(state-rules.table) -> table. The product owner certifies the table on
the graph and a new table ships as a new graph version, never as a code change. Wire the table node
under the graph's island so that no node is left unconnected.

If the function is declared as a TypedLambdaFunction with a PoJo input class, the request body map
is automatically converted to the PoJo at the function boundary.

Result set
----------
Upon successful execution, the function's response body is stored in the "result" parameter, the
response status in "status" and the response headers in "header" in the properties of the node.
The optional output data mapping can copy them to the 'model.' or 'output.' namespace.
The source of an output entry is a constant, a simple-plugin call (f:...), 'result' or 'result.{key}',
'model.{key}' or this node's own namespace - 'input.*' is valid only on the input side. To echo a request value, stage it at a
graph.data.mapper node (input.body.id -> model.id) and map model.id out; any other source fails the
node with an error that names the entry.

Example:
\`\`\`
output[]=result -> model.soap_request_payload
\`\`\`

Timeout
-------
The function call uses the graph instance's time-to-live from "model.ttl" (default 30000 ms).

This deadline bounds the event call to the composable function - it cannot reach inside a
generic function. When the function has its own downstream timeout contract, express it in the
input data mapping. For example, the AsyncHttpClient (async.http.request) takes its HTTP timeout
from the "x-ttl" key-value under "headers" in milliseconds, e.g. \`text(5000) -> headers.x-ttl\`
(see tutorial 13).

Exception handling
------------------
If the function throws an exception (e.g. AppException with a status code) or the call times out,
the "error" and "status" parameters of the node are set (plus "stack" when the failure carries a
stack trace). When the node has an "exception" property, the graph jumps to that error handler
node. Otherwise, the error is returned as the graph output.

When traversal jumps to the handler, the engine also stages a generic exception context that
does not name the failing node:

- error.source  - the failing node's alias
- error.code    - the status code
- error.message - the error message
- error.stack   - the stack trace, when the failure carries one

so ONE handler node can serve the "exception" route of every node in the graph - a graph.task
node, an API fetcher and an extension can all share the same handler, and error.source tells
them apart. Anchor a shared handler from an island (root -> island -> handler) because it is
reached by jumping, and note that a node is visited at most once per run unless RESET. The
alias 'error' is reserved for this namespace - probe it in a dry-run session with
"inspect error". See "describe skill graph.api.fetcher" for the canonical bounded-retry handler.

Example
-------
\`\`\`
create node prepare-soap-request
with type Task
with properties
task=v1.prepare.soap.request
input[]=input.body -> *
input[]=input.header.hello -> header.hello
output[]=result -> model.soap_request_payload
skill=graph.task
\`\`\`
`,gt=`Import a graph model or a node
------------------------------
Load an exported graph model into your session as a draft for review and
update, or copy a single node from another graph model.

Syntax
------
\`\`\`
import graph from {name}
import node {node-name} from {graph-name}
\`\`\`

Example
-------
\`\`\`
import graph from helloworld
import node fetcher from helloworld
\`\`\`

Notes
-----
- The name uses letters, digits and hyphen; do not add a ".json" extension.
- 'import graph' looks in the Playground temp folder first (where
  'export graph' writes). When the file is not there, it falls back to the
  graph models deployed with the application. The message "Graph model not
  found in /tmp/graph/... Found deployed graph model" is this normal
  fallback, not an error - the deployed model is imported as your draft.
- 'import node' copies one node (its type and properties, not its
  connections) from an exported graph model in the temp folder - export the
  source graph first. If a node with the same name already exists in your
  draft, it is overwritten.
- Best practice: publish a common graph model holding reusable nodes
  (modules and skills) so team members can import them into their own
  graph models.
- In the Playground UI, the Graph view's "Import Graph" button (also on the
  empty canvas) and a graph JSON file dropped on the canvas import a model
  from your computer as your draft (POST /api/graph/import/{session-id}).
  The file must be a JSON object with a "nodes" section; "connections" is
  optional (a work in progress may have none); any other top-level section
  is refused by name, and a node without alias or types is refused too.
  CompileGraph remains the quality gate when the model is deployed.
- An import replaces the draft of every member of a shared session, like a
  command, and clears a graph instance; a corrupt model reports "Graph model
  not imported" with the reason. The UI asks before replacing a loaded graph.
`,_t=`Inspect the state machine
-------------------------
Read a value from the current graph instance's state machine: node
properties, and the input, output and model namespaces.

Syntax
------
\`\`\`
inspect {key}
\`\`\`

\`{key}\` is a placeholder - substitute your key and do not type the braces.
A whole namespace (input | output | model | error) is also valid, e.g.
'inspect output'.

After a failed node routes to its exception handler, 'inspect error' shows the staged
exception context - error.source (the failing node), error.code, error.message and
error.stack when available. When the failing node is later retried successfully, the
context resolves: code becomes 200, the source stays, and the failure details are removed
- so an empty context means nothing failed, {source, code: 200} means recovered, and a
full context means an outstanding failure. The 'error' namespace is a first-class
state-machine citizen like 'model', which is why 'error' is a reserved node alias.

Example
-------
\`\`\`
inspect output
inspect input.body.user_id
inspect model.some_variable
inspect output.body.some_key
inspect book.price
inspect error
inspect error.source
\`\`\`

Notes
-----
- Requires a graph instance (see 'help instantiate').
- Keys may be composite (dot-bracket), e.g. output.body.profile[0].name.
- A node's properties and results are addressed by node name, e.g.
  book.price or fetcher.result.name.
- A value too large for the console is redirected: the reply prints a
  GET /api/inspect/... URL to download it instead.
`,vt=`Instantiate a graph instance
----------------------------
Create a runnable instance of the current graph model, optionally seeded
with mock input for development and testing. Required before the 'run',
'execute' and 'inspect' commands. This is a multi-line command: enter all
lines as one block.

Syntax
------
\`\`\`
instantiate graph
{constant} -> input.body.{key}
{constant} -> input.header.{key}
{constant} -> model.{key}
\`\`\`

Example
-------
\`\`\`
instantiate graph
int(100) -> input.body.profile_id
text(application/json) -> input.header.content-type
text(world) -> model.hello
\`\`\`

Notes
-----
- The seed lines are optional. Each line assigns a constant (text(...),
  int(...), boolean(...), etc.) to the input.body, input.header or model
  namespace - no other targets are accepted. The model namespace is the
  state machine; seed it only to emulate model variables.
- Seed keys may be composite (dot-bracket), so nested mock payloads seed
  directly:

\`\`\`
instantiate graph
text(Peter) -> input.body.profile.name
text(100 World Blvd) -> input.body.profile.address1
\`\`\`

- The graph must have a root node and an end node.
- Instantiating replaces any previous instance of your session.
- The reply reports the number of mock entries loaded and the instance's
  model.ttl (default 30000 ms), the execution time budget - seed model.ttl
  to change it.
- 'start' is an alias of 'instantiate'.
- To mock a large input.body with a JSON payload, see 'help upload'.
`,yt=`List nodes, connections, graphs or flows
----------------------------------------
Show all nodes or all connections of the current graph model - or discover
the deployable graph models and Event Script flows of this server.

Syntax
------
\`\`\`
list nodes
list connections
list graphs
list flows
\`\`\`

Notes
-----
- 'list graphs' (discovery, read-only) enumerates the deployable graph
  models - the valid extension={graph-id} delegation targets - each with
  its root node's "purpose" property, so the listing reads as living
  documentation of the enterprise knowledge on this server. Follow up
  with 'describe graph {graph-id}' for a model's input/output contract.
- 'list flows' (discovery, read-only) enumerates the Event Script flows -
  the valid extension=flow://{flow-id} delegation targets.
- 'list nodes' prints each node with its type: the root node first, the end
  node last, and the other nodes in alphabetical order. A missing root or
  end node is flagged with "(does not exist)".
- 'list connections' prints one line per connection with its relation
  label(s).
- Use 'describe node {name}' for the full detail of a single node (see
  'help describe').
`,bt=`Graph set packaging
-------------------
Pack graph models into one graph set - a <set>.pack file checked by the
deployment gate as it is packed - inspect a set, and deploy it all or none.
This topic explains the Playground panel, the command line and the manifest;
there is no console command behind it.

What a graph set is
-------------------
- One file, <set>.pack, holds one or more graph models. The file name
  without the extension is the set name (letters, digits, '_' and '-').
- Each graph is an entry named <graph-id>.json, and the root node's "name"
  must equal the graph id - the rule of 'export graph as'.
- The manifest carries text fields: 'set' (the set name), 'version',
  'description', 'author', the optional 'graph_id' (the set's entry-point
  graph) and anything else you add; 'format' and 'format_version' are the
  packager's. Nothing comes from the clock: the same graphs and fields
  always give the same bytes, so one file can be signed (a detached
  <set>.pack.sig beside it) and promoted.
- A set of one graph is valid: packing a graph alone is how one graph is
  signed. Only an empty set is refused.

The panel
---------
Open it from the Tools menu: "Graph set packaging". It takes the console's
place; Esc or Cancel gives it back, and the list survives a close.

- Drop several <graph-id>.json files on the panel, or browse for them, and
  "Add current graph" adds the graph in the Graph view named after its id.
  Each entry shows its node and connection counts, and what the engine
  would refuse is flagged in place - an id that breaks the file-name rule,
  a root node named differently from its id, a duplicate id - so the list
  is fixed before anything is packed.
- Name the set (required) and fill the manifest fields you want; a blank
  value is left out, and 'graph_id' must name one of the entries.
- "Pack and download" has the engine pack the set (POST /api/graph-set/pack):
  every graph passes the deployment gate first, and a set that breaks a
  rule is refused with every reason. The file is saved through the
  browser's "save as" dialog, or into its download folder.
- Drop a .pack file on the panel to inspect it (POST /api/graph-set/unpack):
  the manifest and the graphs appear; "Import as draft" loads one graph as
  your session's draft (the UI asks before replacing a loaded graph), and
  "Edit as new set" loads the whole set into the editor to pack it again.

The command line
----------------
The graph packager does the same from a build pipeline - a Java jar
(helpers/graph-packager) or a Rust binary (tools/graph-packager):

\`\`\`
graph-packager pack    --set {name} [--manifest key=value]... [--out {dir}] {graph.json}... | {folder}
graph-packager pack    --set {name} --from-manifest graphs.yaml [--manifest key=value]... [--out {dir}]
graph-packager unpack  {file.pack} --out {dir}
graph-packager inspect {file.pack} [--json]
\`\`\`

'pack' prints the SHA-256 for a signer; exit code 1 is a refused input and
2 an I/O or format error. Both engines, and the panel, pack the same graphs
and fields to the same bytes.

Deploying a set
---------------
List the set in the deployment manifest beside the loose graphs, and name a
file:/ folder the application can write, where the set is unpacked before
the gate reads it:

\`\`\`yaml
location: 'classpath:/graph'      # where <set>.pack is read from
sets:
  - 'my-set'
unpack: 'file:/tmp/graph/unpacked'
\`\`\`

- A set registers all of its graphs or none; the ERROR in the startup log
  names every graph the gate refused.
- A duplicate id that involves a set is logged as an ERROR, and the later
  copy in compile order wins (a manifest's sets follow its loose graphs).
- 'list graphs' shows a set's graphs with their set and version, and
  'import graph from {id}' finds the unpacked copy.
- A deployment is still a restart: the sets are re-read at every start, and
  the loader's generated manifest in the unpack folder records what it
  deployed and which files it wrote.

See also 'help export', 'help import' and 'help tutorial 2', and the guides:
the canonical package format and the configuration reference.
`,xt=`Run a graph instance
--------------------
Traverse the current graph instance from the root node to the end node,
executing every node that has a skill along the way.

Syntax
------
\`\`\`
run
\`\`\`

Notes
-----
- Requires a graph instance (see 'help instantiate').
- Before traversal begins, the graph is checked against the same whole-graph
  rules that the CompileGraph deployment gate enforces (the suspend/resume
  contract). Draft authoring allows partial models, but a runnable graph must
  honor these rules - a violation is reported as "Unable to run - <reason>"
  and the run is aborted.
- Traversal starts at the root node. Multiple outgoing connections fork into
  parallel branches (synchronize them with graph.join); each node executes
  at most once per run (loop guard).
- Every run ends with either "Graph traversal completed in N ms" or
  "Graph traversal aborted: <reason>" - every abort names its reason (a node's
  error, a pre-run gate rule, the deadline).
- 'run' may be repeated on the same instance: each run clears the visited
  set and the output namespace, but model values persist across runs -
  instantiate again for a completely fresh state.
- Use 'seen' to list the nodes visited by the last run, and 'inspect' to
  read the results (e.g. 'inspect output.body').
`,St=`Display nodes that have been 'seen'
-----------------------------------
List the nodes of the current graph instance that have been seen - visited
by graph traversal or executed directly.

Syntax
------
\`\`\`
seen
\`\`\`

Notes
-----
- Requires a graph instance (see 'help instantiate').
- Covers nodes visited by 'run' and nodes tested with 'execute'.
- The visited set is cleared at the start of each run.
`,Ct=`Session commands
----------------
Manage your Playground session and collaborate with other users by
subscribing to a primary session, so both users see and drive the same graph.

Syntax
------
\`\`\`
session                    show this session's id and subscriptions
session subscribe {id}     mirror a primary session into yours
session unsubscribe        detach from the session you subscribed to
session reset              restart your session
\`\`\`

Example
-------
\`\`\`
> session
Session ws-178443-2 started since 2026-06-02 10:20:32.054
subscribed by [ws-485844-4]
\`\`\`

Notes
-----
- 'session' shows the session id and start time, the session you subscribed
  to (if any), and the sessions subscribed to yours.
- Subscribing mirrors commands both ways: input commands from either user
  run in both sessions, keeping the graphs in sync. A mock-data upload
  (POST /api/mock/{id}, see 'help upload') travels the same way: it loads
  into every member's graph instance. On subscribe the graphs
  are aligned - if the primary session is empty, your draft is pushed to it;
  otherwise its graph replaces your draft.
- You can subscribe only to a primary session (one that has not itself
  subscribed to another), and never to yourself. If you are already
  subscribed, do 'session reset' before subscribing to another session.
- 'session unsubscribe' decouples your session from the one you subscribed
  to; your graph is retained so you can continue editing. A primary session
  gets "Nothing to unsubscribe".
- 'session reset' restarts your session with an empty draft graph. As a
  primary session it disconnects all subscribers (they keep their own
  graphs); as a subscriber it unsubscribes first. Export first if you want
  to keep your draft (see 'help export').
- The companion REST endpoints reject 'session subscribe', 'session
  unsubscribe' and 'session reset': a companion is an assistant to a
  session, not a session of its own. Only the read-only 'session' status
  query works there - session topology is managed from a
  WebSocket-connected session (the browser console) only.
`,wt=`Tutorial 1
----------
Welcome to the MiniGraph Playground, the self-service user interface for creating
applications with the Active Knowledge Graph.

In this tutorial, you will create the simplest possible application: a graph model
that returns a "hello world" message.

Exercise
--------
If you can see this page, you have successfully started the MiniGraph Playground in a
browser and connected to a designer workbench session.

If your session is disconnected, select the "Tools" dropdown in the top-right corner,
click MiniGraph's start toggle and select "MiniGraph".

Create the starting point of a graph
------------------------------------
**Create a root node** — the starting point of every graph model.
Select multiline and enter the following command in the bottom-right input box.

\`\`\`
create node root
with type Root
with properties
purpose=Tutorial one to return a 'hello world' message
\`\`\`

The console displays:

\`\`\`
> create node root...
Graph with 1 node described in /api/graph/model/ws-875677-2/165-1
\`\`\`

A drawing appears on the right-hand side under the "Graph" tab: a graph with a single
node called "root" has been created.

\`ws-875677-2\` is the session ID of the workbench.
\`165-1\` is a random number for the session that you can ignore.

Create an end node
------------------
An end node is the exit point of a graph model. Enter the following to create one.

\`\`\`
create node end
with type End
with properties
skill=graph.data.mapper
mapping[]=text(hello world) -> output.body
\`\`\`

The console displays:

\`\`\`
> create node end...
Graph with 2 nodes described in /api/graph/model/ws-875677-2/061-2
\`\`\`

The \`skill=graph.data.mapper\` line assigns the data mapper skill to the end node.
A data mapper node performs data mapping when it executes.

The mapping statement \`mapping[]=text(hello world) -> output.body\` maps the constant
"hello world" to \`output.body\` — the response payload when the graph is executed.
The \`[]\` suffix means \`mapping\` is a list: each \`mapping[]=\` line appends one statement.

MiniGraph uses the same data mapping syntax as Event Script. For a quick reference,
enter "help graph-data-mapper" in the console.

First attempt to run the graph
------------------------------
To run a graph model, first create an instance of it with the \`instantiate graph\` command.

The console displays:

\`\`\`
> instantiate graph
Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms
\`\`\`

Now try to run the graph by entering the \`run\` command.

The console displays:

\`\`\`
> run
Walk to root
\`\`\`

The system starts graph traversal from the starting point, i.e. the root node —
and then nothing happens.

What is missing?
----------------
An Active Knowledge Graph is a "property graph" that contains one or more "active"
nodes. An active node carries a "skill" that is backed by a composable function.

The system traverses the graph from the root node. Nothing happened because there is
no further node to reach after the root node: the two nodes are not yet connected,
so traversal stops before it can reach the end node.

Connecting nodes
----------------
Enter the following command to connect the root node to the end node.

\`\`\`
connect root to end with done
\`\`\`

The console displays:

\`\`\`
> connect root to end with done
node root connected to end
Graph with 2 nodes described in /api/graph/model/ws-875677-2/551-3
\`\`\`

The graph drawing on the right panel is updated.

Running the graph
-----------------
You now have a graph with a starting point and an ending point, where one node
carries a skill — the end node with its data mapping statement.

Instantiate the graph again and run it by entering the following commands.

\`\`\`
instantiate graph
run
\`\`\`

The console displays:

\`\`\`
> instantiate graph
Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms
> run
Walk to root
Walk to end
Executed end with skill graph.data.mapper in 1.736 ms
{
  "output": {
    "body": "hello world"
  }
}
Graph traversal completed in 9 ms
\`\`\`

Congratulations — you have created your first working MiniGraph.
It returns "hello world" when it runs.

Export the graph
----------------
You may now export the graph so that you can deploy it later.

Enter the export command below:

\`\`\`
export graph as tutorial-1
\`\`\`

This exports the graph model in JSON format with the name \`tutorial-1\`
as "/tmp/graph/tutorial-1.json".

The console displays:

\`\`\`
> export graph as tutorial-1
Added name=tutorial-1 to Root node
Graph exported to /tmp/graph/tutorial-1.json
Described in /api/graph/model/tutorial-1/436-4
\`\`\`

Note that the system adds the graph name (its unique "id") to the root node.
This prevents you from accidentally overwriting a different graph model.

Help pages
----------
To learn more about each command used in this tutorial, enter:

\`\`\`
help create
help connect
help instantiate
help run
help export
\`\`\`

Summary
-------
In this tutorial, you created the simplest graph model — it returns a "hello world"
message when its graph API endpoint is called — exported it, and tried some help pages.

Well done. Let's move on to "Tutorial 2".
`,Tt=`Tutorial 10
-----------
In this tutorial, you will create a graph model that uses another graph model as an extension.

Exercise
--------
You will use an existing graph model as an extension, then create a new graph model that calls it.

To clear the previous graph session, click the Tools button in the top-right corner and click the
"Stop" and "Start" toggle button. A new graph session will start.

What is a graph extension?
--------------------------
A graph extension is a graph model built to serve some logic that another graph model can reuse.

The \`extension\` property of a graph.extension node names a **deployed** graph model — one compiled
at application startup from the \`resources/graph\` folder (the same ids callable at
POST /api/graph/{graph-id}). A session draft is not addressable as an extension: export and deploy
it first.

Import tutorial 3 as an extension
---------------------------------
Enter the following to import tutorial 3. Note that tutorial-3.json is preloaded into the
\`resources/graph\` folder.

\`\`\`
> import graph from tutorial-3
Graph model not found in /tmp/graph/tutorial-3.json
Found deployed graph model in classpath:/graph
Please export an updated version and re-import to instantiate an instance model
Graph model imported as draft
\`\`\`

Once the graph model is imported, start the graph with mock data.

\`\`\`
start graph
int(100) -> input.body.person_id
\`\`\`

Then do a 'dry-run'.

\`\`\`
> run
Walk to root
Walk to fetcher
Executed fetcher with skill graph.api.fetcher in 0.982 ms
Walk to end
{
  "output": {
    "body": {
      "address": "100 World Blvd",
      "name": "Peter"
    }
  }
}
Graph traversal completed in 2 ms
\`\`\`

You can see that it fetches data using the input parameter (person_id=100) and returns the name
and address of the person. This is the behavior your new graph will reuse.

Restart playground session
--------------------------
You will clear the current graph session — click the Tools button in the top-right corner and
click the "Stop" and "Start" toggle button. A new graph session will start.

Create a root node and an end node
----------------------------------
You will create a new graph model with a root node and an end node.

\`\`\`
create node root
with type Root
with properties
name=tutorial-10
purpose=Demonstrate the use of graph extension
\`\`\`

\`\`\`
create node end
with type End
\`\`\`

Create a node to use an extension
---------------------------------
Enter the following to create an extension node. The skill is 'graph.extension' and the
'extension' property names the deployed graph model 'tutorial-3'.

The input mapping sets the input parameter(s) of the extension, which is itself a graph model.
The output mapping sets the result from the extension to the output payload.

\`\`\`
create node extension
with type Extension
with properties
skill=graph.extension
extension=tutorial-3
input[]=input.body.person_id -> person_id
output[]=result -> output.body
\`\`\`

Connect the nodes to complete the graph model
---------------------------------------------

\`\`\`
connect root to extension with run
connect extension to end with finish
\`\`\`

Test the graph model
--------------------
Enter the following to instantiate the graph model with mock input.

\`\`\`
instantiate graph
int(100) -> input.body.person_id
\`\`\`

Then do a 'dry-run'.

\`\`\`
> run
Walk to root
Walk to extension
Executed extension with skill graph.extension in 19.013 ms
Walk to end
{
  "output": {
    "body": {
      "address": "100 World Blvd",
      "name": "Peter"
    }
  }
}
Graph traversal completed in 20 ms
\`\`\`

The input of the current graph instance is mapped as an input parameter to the extension
'tutorial-3', and the result is mapped as the output of the graph.

If you inspect the extension node, you will see:

\`\`\`
> inspect extension
{
  "inspect": "extension",
  "outcome": {
    "result": {
      "address": "100 World Blvd",
      "name": "Peter"
    },
    "live": true,
    "target": "tutorial-3",
    "status": 200
  }
}
> inspect output
{
  "inspect": "output",
  "outcome": {
    "body": {
      "address": "100 World Blvd",
      "name": "Peter"
    }
  }
}
\`\`\`

Check the application log
-------------------------
Complete telemetry information is shown in the application log. You will see that 'tutorial-3' is
invoked as an extension and that it fetches data from the data provider with the input parameter
'person_id'.

\`\`\`
Call extension tutorial-3, ttl=30000
GET http://127.0.0.1:8085/api/mdm/profile/100, with [person_id], ttl=30000
\`\`\`

This is a small example, but it demonstrates the pattern: a typical main graph model uses one or
more extensions for API data fetching, then performs decision-making using the retrieved data.

Reusability
-----------
Graph extension promotes reusability. Common use cases can be built as graph models and made
available as "extensions" for other graph models to use.

Export the graph model
----------------------
Now you may save the graph model by exporting it.

\`\`\`
> export graph as tutorial-10
Graph exported to /tmp/graph/tutorial-10.json
Described in /api/graph/model/tutorial-10/286-8
\`\`\`

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-10.json" to your application's
\`resources/graph\` folder. You can then test the deployed model with a curl command.

\`\`\`
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-10 \\
  -H "Content-Type: application/json" \\
  -d '{ 
    "person_id": 100
}'
\`\`\`

Summary
-------
In this tutorial, you have created a graph model that uses a graph extension: the 'extension'
property names a deployed graph model, input mappings feed its input.body, and the extension's
output.body comes back as the node's result.
`,Et=`Tutorial 11
-----------
In this tutorial, you will create a graph model that uses an "event flow" as an extension.

Pre-requisite
-------------
You would need some working knowledge of Event Script. For more details, see the
Event Script guide: https://accenture.github.io/mercury-composable/guides/event-script/

Assuming you already know how to create an event flow (configuration plus composable functions as
tasks), it is easy to use an event flow as an extension.

What is a flow extension?
-------------------------
A flow extension is an event flow built to serve some logic that a graph model can reuse. The same
graph.extension skill from tutorial 10 is used — only the target changes: the "flow://" protocol
prefix tells the system to execute an event flow instead of another graph model.

Import the graph model from tutorial 10
---------------------------------------
In tutorial 10, you created a main graph that calls another graph as an extension and exported it
as tutorial-10. Import it back as your starting point:

\`\`\`
import graph from tutorial-10
\`\`\`

The import loads the version you exported in tutorial 10 from the temporary graph folder — or
falls back to the preloaded copy in classpath:/graph if you have not exported one.

Edit the root node
------------------
Enter 'edit node root' and copy-n-paste the content into the input box. Change the name and
purpose for tutorial 11.

\`\`\`
update node root
with type Root
with properties
name=tutorial-11
purpose=Demonstrate the use of flow extension
\`\`\`

Edit the extension node
-----------------------
Enter 'edit node extension' and copy-n-paste the content into the input box. Update the extension
to "flow://flow-11" and change the input statements to pass "hello" and "message" as parameters.
The flow protocol prefix tells the system to execute the flow with the identifier "flow-11".

\`\`\`
update node extension
with type Extension
with properties
extension=flow://flow-11
input[]=input.body.hello -> hello
input[]=input.body.message -> message
output[]=result -> output.body
skill=graph.extension
\`\`\`

About flow 11
-------------
For your convenience, "flow-11" is preloaded. You can review the configuration files "flows.yaml"
and "flow-11.yml" in the resources folder. The event flow "flow-11" is an echo program: the task
"no.op" echoes everything from the input and passes it as output. Below is an extract of the event
flow's first task.

\`\`\`yaml
tasks:
  - input:
      # pass all input parameters as arguments
      - 'input.body -> *'
    process: 'no.op'
    output:
      - 'result -> output.body'
    description: 'echo everything in the input payload'
    execution: end
\`\`\`

Perform a dry-run
-----------------
To test the updated graph model, instantiate the graph with the two inputs "hello" and "message"
as follows:

\`\`\`
instantiate graph
text(world) -> input.body.hello
text(this is a good day) -> input.body.message
\`\`\`

Then enter 'run' to execute the graph.

\`\`\`
> start graph...
Graph instance created. Loaded 2 mock entries, model.ttl = 30000 ms
> run
Walk to root
Walk to extension
Executed extension with skill graph.extension in 5.46 ms
Walk to end
{
  "output": {
    "body": {
      "hello": "world",
      "message": "this is a good day"
    }
  }
}
Graph traversal completed in 7 ms
\`\`\`

You can also check the application log, where telemetry and tracing information are shown.

\`\`\`
Call extension flow://flow-11, ttl=30000
{trace={path=/graph/playground, service=graph.extension...
{trace={path=/graph/playground, service=no.op...
{trace={path=/graph/playground, service=task.executor...
{trace={path=/graph/playground, service=event.script.manager...
\`\`\`

This validates that the event flow instance for "flow-11" was executed by the graph instance for
tutorial-11.

Why extend a graph model with an event flow?
--------------------------------------------
While the graph extension discussed in tutorial 10 can compose sophisticated and powerful graph
models, extending a graph with an event flow lets you go beyond API fetching, data mapping,
computation and decision-making.

With an event flow, you can model very complex transaction processing in "pro-code". Combining
graph modeling with Event Script programming gives you the best of both worlds — no-code and
pro-code — to tackle the most demanding use cases.

Export the graph model
----------------------
Now you may save the graph model by exporting it.

\`\`\`
> export graph as tutorial-11
Graph exported to /tmp/graph/tutorial-11.json
Described in /api/graph/model/tutorial-11/794-6
\`\`\`

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-11.json" to your application's
\`resources/graph\` folder. You can then test the deployed model with a curl command.

\`\`\`
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-11 \\
  -H "Content-Type: application/json" \\
  -d '{ 
    "hello": "world",
    "message": "this is a good day"
}'
\`\`\`

Summary
-------
In this tutorial, you have used an event flow as an extension to a graph model, selected with the
flow protocol prefix "flow://". The delegation contract is the same as for a sub-graph: the input
mappings feed the flow's input.body, and the flow's output.body comes back as the node's result.
`,Dt=`Tutorial 12
-----------
In this tutorial, you will create a graph model with custom error handling.

Exercise
--------
You will import tutorial 3 and add an error-handler node that retries an API failure.

To clear the previous graph session, click the Tools button in the top-right corner and click the
"Stop" and "Start" toggle button. A new graph session will start.

Import tutorial 3 as a template
-------------------------------
Enter the following to import tutorial 3. Note that tutorial-3.json is preloaded into the
\`resources/graph\` folder.

\`\`\`
> import graph from tutorial-3
Graph model not found in /tmp/graph/tutorial-3.json
Found deployed graph model in classpath:/graph
Please export an updated version and re-import to instantiate an instance model
Graph model imported as draft
\`\`\`

Update the root node
--------------------
Enter the following to update the root node. It assigns the skill "graph.data.mapper" to the node
and maps the input parameter "exception" to the model variable with the same name.

The \`f:defaultValue()\` plugin function sets the variable "model.exception" to false when the input
parameter is not given.

We will use the model.exception parameter to trigger a simulated exception in the mdm-profile
service.

\`\`\`
update node root
with type Root
with properties
mapping[]=f:defaultValue(input.body.exception, boolean(false)) -> model.exception
name=tutorial-12
purpose=Demonstrate custom error handling
skill=graph.data.mapper
\`\`\`

Update the dictionary
---------------------
For person-address, you will add the input parameter \`exception:false\`, where ":false" is the
default value of the parameter when it is not given.

\`\`\`
update node person-address
with type Dictionary
with properties
input[]=person_id
input[]=exception:false
output[]=response.profile.address -> result.address
provider=mdm-profile
purpose=address of a person
\`\`\`

and do the same for person-name

\`\`\`
update node person-name
with type Dictionary
with properties
input[]=person_id
input[]=exception:false
output[]=response.profile.name -> result.name
provider=mdm-profile
purpose=name of a person
\`\`\`

Update the data provider
------------------------
You will add the input data mapping \`exception -> header.x-exception\` to the mdm-profile node. The
input parameter "exception" is used to set the HTTP request header "X-Exception".

\`\`\`
update node mdm-profile
with type Provider
with properties
feature[]=log-request-headers
feature[]=log-response-headers
input[]=text(application/json) -> header.accept
input[]=exception -> header.x-exception
input[]=person_id -> path_parameter.id
method=GET
purpose=Master Data Management's profile management endpoint
url=http://127.0.0.1:\${rest.server.port:8080}/api/mdm/profile/{id}
\`\`\`

Update the fetcher node
-----------------------
You will add the input data mapping \`model.exception -> exception\` to set the parameter
"exception" when retrieving the two data dictionary items (person-name and person-address).

You also add the property \`exception=error-handler\`. This tells the system to route the flow to
the "error-handler" node when a call fails, instead of aborting the graph traversal.

\`\`\`
update node fetcher
with type Fetcher
with properties
dictionary[]=person-name
dictionary[]=person-address
exception=error-handler
input[]=input.body.person_id -> person_id
input[]=model.exception -> exception
output[]=result.name -> output.body.name
output[]=result.address -> output.body.address
skill=graph.api.fetcher
\`\`\`

The dev-mode mock endpoint (mock.mdm.profile) answers with a 401 "simulated
exception" when the request header x-exception is true. In the Java example it
reads like this (the Rust example's mock endpoint does the same):

\`\`\`java
@Override
public Object handleEvent(Map<String, String> headers, AsyncHttpRequest input, int instance) {
    if ("true".equals(input.getHeader("x-exception"))) {
        throw new AppException(401, "simulated exception");
    }
    // for simplicity, business logic not shown here
}
\`\`\`

Create the error-handler node
-----------------------------
You will now create the error-handler node referenced by the fetcher above.

When the "exception" property is configured on a fetcher, a failed call — an error status is
always a value of 400 or higher — does not abort the graph traversal: the engine sets the node's
"status" and "error" variables, skips its output mappings, and routes the flow to the named error
handler.

This handler is GENERIC: it never names the failing node. When a failed node routes to its
"exception" handler, the engine stages the exception context in the state machine —
error.source (the failing node's alias), error.code (the status code), error.message and
error.stack when available — and every statement command resolves {dynamic variables}, so the
handler reads and jumps back through the context ('inspect error' shows it in a dry-run
session; the node alias "error" is reserved for this namespace).

The handler's statements run in order:

1. The first IF tests "{error.code}". It is good practice to test for exactly 200 so an
   unintended configuration error cannot slip through. On HTTP-200 the THEN branch jumps to the
   end node — a taken node-jump ends the statement list immediately. Otherwise the ELSE branch
   resolves to "next" and falls through to the following statements.
2. RESET comes **first among the action statements**: it clears the run-once guard and state of a
   comma-separated list of nodes so they can be executed again — here "{error.source}" (whichever
   node routed here) and the error-handler itself (a node may reset itself because the run-once
   mark is set before its statements execute). Placing RESET early guarantees it runs on every
   path — a later taken IF jump would skip it — and everything the node stores afterwards (such
   as the pending DELAY) survives the self-wipe. Keep RESET **after** any check that reads state
   it would wipe: the status IF above must run first, because RESET clears the failing node's
   "status" and an IF on a wiped variable aborts the run (the staged "error.*" context itself is
   not node state, so it survives).
3. The two MAPPING statements increment the retry counter "model.attempts" (\`f:defaultValue()\`
   seeds it to 0 on the first pass). The "model" namespace is not touched by RESET.
4. The second IF bounds the retry loop: after 3 attempts it jumps to the "clear-exception" node.
5. "NEXT: {error.source}" tells the traversal system to jump back to the failing node — whichever
   node routed here. Unlike a taken IF jump, NEXT does not stop the statement list — the jump is
   applied after the whole list completes.
6. "DELAY: 50" pauses for 50 milliseconds after this node completes, before the next retry. Pacing
   retries is a best practice: it avoids very rapid retries that can cause a "recovery storm" — an
   unintended denial-of-service attack on the target service. (A DELAY value may also be a dynamic
   variable, e.g. "DELAY: {model.backoff}" for a computed backoff.)

After the successful retry, the virtual "error" node reports the RECOVERY instead of the stale
failure: "inspect error" shows code=200 with the source kept and the failure details removed.
An empty context means nothing failed; a full context means an outstanding failure.

\`\`\`
create node error-handler
with type Decision
with properties
skill=graph.math
statement[]='''
IF: {error.code} == 200
THEN: end
ELSE: next
'''
statement[]=RESET: {error.source}, error-handler
statement[]=MAPPING: f:defaultValue(model.attempts, int(0)) -> model.attempts
statement[]=MAPPING: f:add(model.attempts, int(1)) -> model.attempts
statement[]='''
IF: {model.attempts} >= 3
THEN: clear-exception
ELSE: next
'''
statement[]=NEXT: {error.source}
statement[]=DELAY: 50
\`\`\`

Create the clear-exception node
-------------------------------
In the clear-exception node, the RESET comes first (nothing before it reads node state), clearing
the failing node — again via the dynamic "{error.source}" reference, since the exception context
stays readable for the rest of the run — and the clear-exception node itself so that the system
can execute them again. You then set the variable "model.exception" to false so that the mock
service returns a normal response instead of an exception, and clear "model.attempts" to zero.

\`\`\`
create node clear-exception
with type Decision
with properties
skill=graph.math
statement[]=RESET: {error.source}, clear-exception
statement[]=MAPPING: boolean(false) -> model.exception
statement[]=MAPPING: int(0) -> model.attempts
\`\`\`

Connections for error-handler and clear-exception nodes
-------------------------------------------------------
Create the connections to complete the retry loop.

\`\`\`
connect error-handler to fetcher with retry
connect clear-exception to fetcher with reset
\`\`\`

Do a dry-run
------------
Enter the following to start the graph with mock input data. You are setting the integer 100 to
person_id and the boolean value "true" to exception in the input payload.

\`\`\`
start graph
int(100) -> input.body.person_id
boolean(true) -> input.body.exception
\`\`\`

Execute the run command.

\`\`\`
> run
Walk to root
Executed root with skill graph.data.mapper in 0.231 ms
Walk to fetcher
Walk to dictionary
Executed dictionary with skill graph.island in 0.014 ms
Executed fetcher with skill graph.api.fetcher in 21.83 ms
Walk to error-handler
Executed error-handler with skill graph.math in 52.242 ms
Walk to fetcher
Executed fetcher with skill graph.api.fetcher in 8.025 ms
Walk to error-handler
Executed error-handler with skill graph.math in 51.824 ms
Walk to fetcher
Executed fetcher with skill graph.api.fetcher in 8.264 ms
Walk to error-handler
Executed error-handler with skill graph.math in 51.837 ms
Walk to clear-exception
Executed clear-exception with skill graph.math in 0.132 ms
Walk to fetcher
Executed fetcher with skill graph.api.fetcher in 0.547 ms
Walk to end
{
  "output": {
    "body": {
      "address": "100 World Blvd",
      "name": "Peter"
    }
  }
}
Graph traversal completed in 201 ms
\`\`\`

The graph traversal log shows that the "error-handler" node executed 3 times before the
clear-exception node ran. After the exception is cleared, the mock service returns a correct
result set as "output".

Export the graph model
----------------------
Now you may save the graph model by exporting it.

\`\`\`
> export graph as tutorial-12
Graph exported to /tmp/graph/tutorial-12.json
Described in /api/graph/model/tutorial-12/591-5
\`\`\`

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-12.json" to your application's
\`resources/graph\` folder. You can then test the deployed model with a curl command.

\`\`\`
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-12 \\
  -H "Content-Type: application/json" \\
  -d '{ 
    "person_id": 100,
    "exception": true
}'
\`\`\`

Summary
-------
In this tutorial, you have used tutorial-3 as a template and enhanced it with custom error
handling.

You have used the keywords "RESET", "NEXT" and "DELAY" to clear the state of visited nodes, to
tell the graph traversal system to route to a specific node, and to introduce an artificial delay
that avoids overwhelming the target service.

IMPORTANT: Graph traversal loops
--------------------------------
The graph traversal system is designed to allow a node to be executed only once per run.

When you use the keyword "RESET: node-name", the "seen" status and all state information are
cleared so that the node can be executed again. This creates the potential for an endless loop in
graph traversal.

Therefore, always include decision logic that bounds the looping or retries — like the
"model.attempts" counter in this tutorial.

As a protection mechanism, the system has built-in loop detection. When a node is executed too
frequently, the graph traversal is aborted.

The default parameters in \`application.properties\` allow 10 visits per second for the same node.

\`\`\`properties
graph.max.loop.interval=1000
graph.node.high.frequency=10
\`\`\`
`,Ot=`Tutorial 13
-----------
In this session, you will create a graph model that invokes a composable function using the
"graph.task" skill. The composable function is the AsyncHttpClient (route "async.http.request")
provided by the platform-core module, turning the task node into an HTTP client by configuration.

Pre-requisite
-------------
You would need some working knowledge of composable functions. A composable function is a
TypedLambdaFunction registered with the PreLoad annotation. For more details, please refer to the
[Developer Guide](https://accenture.github.io/mercury-composable/).

What is a task?
---------------
A task is a node that invokes a composable function through its route name. MiniGraph is designed to be
zero-code with built-in skills for data mapping, decision-making and API fetching. More complex business
logic is delegated to a flow extension or a subgraph (tutorials 10 and 11). A task node sits in between -
it provides a lightweight method to extend a knowledge graph's capability with a small piece of business
logic, without writing a new skill.

In this tutorial, the "small piece of business logic" is not custom code at all - it is the framework's
own AsyncHttpClient. Any function registered in the platform is callable by route name, so the task node
can drive an HTTP call purely by configuration.

Create the graph model
----------------------
Create the root node:

\`\`\`
create node root
with type Root
with properties
name=tutorial-13
purpose=Demonstrate the graph.task skill - invoking a composable function through its route name
\`\`\`

Create the task node. The "task" property is the route name of the composable function:

\`\`\`
create node hello-task
with type Task
with properties
input[]=input.body.person_id -> model.person_id
input[]=text(http://127.0.0.1:\${rest.server.port:8080}) -> host
input[]=text(/api/mdm/profile/{model.person_id}) -> url
input[]=text(GET) -> method
input[]=text(application/json) -> headers.accept
input[]=text(5000) -> headers.x-ttl
output[]=result -> output.body
purpose=Invoke AsyncHttpClient with route 'async.http.request' to fetch a user profile
skill=graph.task
task=async.http.request
\`\`\`

Create the end node and connect the three nodes:

\`\`\`
create node end
with type End
\`\`\`

\`\`\`
connect root to hello-task with run
connect hello-task to end with finish
\`\`\`

For your convenience, this graph model is also preloaded. You can import it with
'import graph from tutorial-13' instead of creating the nodes manually.

About the input data mapping
----------------------------
The input data mapping follows the Event Script syntax and is applied in declaration order:

1. \`input.body.person_id -> model.person_id\` stages a variable in the graph's state machine
   (the \`model.\` namespace). It does not become part of the function's request - it is kept for
   later entries to reference.
2. \`text(/api/mdm/profile/{model.person_id}) -> url\` demonstrates a **dynamic variable**: the
   \`{model.person_id}\` reference inside the text constant is substituted with the model value
   staged by the earlier entry. This is the same idiom as Event Script's
   \`text(Bearer {model.token}) -> headers.Authorization\`.
3. \`text(http://127.0.0.1:\${rest.server.port:8080}) -> host\` demonstrates **environment variable
   substitution**. The \`\${name:default}\` reference is resolved by the configuration system when the
   model is loaded - at 'instantiate graph' for a dry-run and at deployment time for a deployed
   model - so both lanes behave the same. The authored model (and any export) keeps the \`\${...}\`
   placeholder, making the model portable across environments.
4. \`text(application/json) -> headers.accept\` declares the response type this client accepts.
   Always declare it instead of relying on an HTTP library's implicit default - with an explicit
   accept, the profile service replies with \`content-type: application/json\` and the
   AsyncHttpClient decodes the response body into a map.
5. \`text(5000) -> headers.x-ttl\` sets the **HTTP timeout** of the AsyncHttpClient. The graph's
   regular ttl propagation (a node's optional \`ttl\` property, else \`model.ttl\`) bounds only the
   event call to the composable function - it cannot reach inside a generic function, so the
   HTTP client would otherwise run on its own 30-second default. The X-TTL value is expressed
   in **milliseconds**. It also rides the wire as the \`X-TTL\` request header, so a downstream
   Mercury service adopts it as its processing deadline (end-to-end deadline propagation).
6. Any other RHS such as \`url\` and \`method\` is a composite key path in the function's request body.
   RHS \`*\` would map the LHS value as the whole request body, and \`header.{name}\` would set a
   request header of the function call.

About async.http.request
------------------------
The input data mapping above builds a map of key-values. The AsyncHttpRequest class in platform-core
renders that map into an HTTP request through its "fromMap" method at the function boundary. The
commonly used keys are:

\`\`\`
host              target host, e.g. http://127.0.0.1:8080
url               URI path, e.g. /api/mdm/profile/100
method            GET, POST, PUT, DELETE, etc.
headers.{name}    an HTTP request header
headers.x-ttl     HTTP timeout in milliseconds (default 30000); also propagates on the wire
body              the HTTP request body (for POST/PUT)
parameters.query.{name}   a query parameter
\`\`\`

The mock MDM profile service (GET /api/mdm/profile/{id}) is preloaded in dev mode, serving
person IDs 100 and 200.

Perform a dry-run
-----------------
To test the graph model, you can instantiate the graph with mock input as follows:

\`\`\`
instantiate graph
int(100) -> input.body.person_id
\`\`\`

Then enter 'run' to execute the graph.

\`\`\`
> start graph...
Graph instance created. Loaded 1 mock entry, model.ttl = 30000 ms
> run
Walk to root
Walk to hello-task
Executed hello-task with skill graph.task in 18.5 ms
Walk to end
{
  "output": {
    "body": {
      "profile": {
        "id": "100",
        "name": "Peter",
        "address": "100 World Blvd"
      },
      "accounts": ["a101", "b202", "c303", "d400", "e500"],
      "observed_ttl": "5000"
    }
  }
}
Graph traversal completed in 21 ms
\`\`\`

Note that 'instantiate graph' resolved \`\${rest.server.port:8080}\` to the application's actual port,
and the task node resolved \`{model.person_id}\` to 100 before calling the HTTP endpoint. The
"observed_ttl" field is the mock service echoing the \`X-TTL\` request header it received - proof
that the 5000 ms deadline arrived on the wire.

You can also check the application log. Telemetry and tracing information are shown, proving that the
composable function was executed by the graph instance with full trace propagation.

\`\`\`
GraphTask:144 - Call task async.http.request, ttl=30000
Telemetry:81 - {trace={path=/graph/playground, service=graph.task...
Telemetry:81 - {trace={path=/graph/playground, service=async.http.request...
Telemetry:81 - {trace={path=/graph/playground, service=mock.mdm.profile...
\`\`\`

Error handling
--------------
If the composable function throws an exception (e.g. AppException with a status code) or the call times
out, the "error" and "status" parameters of the node are set. You can add an "exception" property to the
task node to route the error to a handler node, e.g. \`exception=on-error\`.

You can see this without any extra configuration - instantiate with an unknown person ID such as
\`int(999) -> input.body.person_id\` and the HTTP error from the profile service becomes the graph
output.

Iterative execution
-------------------
Like the API fetcher and the flow extension, a task node supports iterative fork-join execution with the
"for_each" and "concurrency" properties. Please enter 'describe skill graph.task' for details.

Export the graph model
----------------------
Now you may save the graph model by exporting it.

\`\`\`
> export graph as tutorial-13
Graph exported to /tmp/graph/tutorial-13.json
Described in /api/graph/model/tutorial-13/431-3
\`\`\`

The exported file keeps the \`\${rest.server.port:8080}\` placeholder, so the same model resolves to
the correct port in each environment it is deployed to.

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-13.json" to your application's \`main/resources/graph\`
folder. You can then test the deployed model with a curl command.

\`\`\`
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-13 \\
  -H "Content-Type: application/json" \\
  -d '{
    "person_id": 100
}'
\`\`\`

Summary
-------
In this session, we have discussed the use of the "graph.task" skill to invoke a composable function
through its route name, with Event Script style input and output data mapping. Along the way you used
a model variable as a dynamic variable in a later data mapping entry, and an environment variable
reference that resolves when the model is loaded.

Why invoke a composable function from a graph?
----------------------------------------------
The built-in skills cover data mapping, decision-making, computation and API fetching without writing
any code, and flow extensions or subgraphs handle complex orchestration. A task node completes the
picture - any custom business logic can now be packaged as a composable function and plugged into a
graph as if it were a custom skill. As this tutorial shows, that includes functions the framework
already provides: the AsyncHttpClient became an HTTP client by configuration, with no code at all.
`,kt=`Tutorial 14
-----------
In this session, you will build a purchase workflow with THREE human checkpoints - a customer
orders, the store manager approves (or rejects with a reason, which ends the workflow), the
delivery department releases the shipment, and the parcel ships to the customer. One graph
model, four short runs, one correlation ID.

When to use suspension
----------------------
Any multi-step process that must WAIT mid-way fits this pattern - waiting for a person (an
approval, missing information requested by email) or waiting for another system (a batch job
that takes hours and calls back when done). The workflow pauses instead of ending: its state
is kept in an external store under the business correlation ID - the ticket or order number -
and any application instance resumes it when the reply arrives.

Pre-requisite
-------------
Workflow suspension persists state to an external store through two composable functions. This
tutorial uses the Redis store from the "minigraph-state-redis" extension - the playground
application already includes it, so "v1.redis.persist.model" and "v1.redis.retrieve.model" are
registered automatically. Start a Redis before you run the graph (the "redis-standalone" helper
application works out of the box).

What is workflow suspension?
----------------------------
An approval may take minutes or days. Instead of parking a live graph instance, the graph
persists its workflow state - the "model" namespace - under the business correlation ID and the
run completes normally. A later request with the same correlation ID restores that state and
continues past the checkpoint without re-executing it. Three vocabulary pieces make this work:

1. the "suspend" node - a reserved node name (like root and end) with the "graph.suspend" skill.
   ONE suspend node serves every suspension point in the graph, reached two ways: a CHECKPOINT
   NODE - a working node with a DRAWN EDGE to it - pauses when its skill completes (the edge is
   the declaration, no property needed), and a DECISION NODE pauses by returning "suspend" from
   its IF-THEN-ELSE (the decision is RE-EXECUTED against the new input on every resume)
2. a checkpoint node - any working node (graph.data.mapper here; graph.task, graph.api.fetcher
   and graph.extension work the same way) that draws its checkpoint edge to "suspend" plus a
   continuation edge to the next step; a resumed run continues along the continuation and never
   re-executes the node. A checkpoint never decides - reaching it IS the decision to pause
3. the resume node - the "graph.resume" skill placed right after root; it restores a persisted
   record and continues at the LAST suspension point: past a checkpoint along its continuation,
   or by re-executing the decision that paused the workflow. A fresh transaction flows through -
   either way it sets "model.run" to "resume" or "fresh" so the graph's own logic can react

The graph navigation is:

\`\`\`
root -> resume -> order -> check-approval -> approval -> delivery -> ship -> end
              (checkpoint)     |      \\-> manager-reject -> end     (order, approval and
                               |                                     delivery draw edges
                               +--returns 'suspend' when no valid    to suspend)
                                  decision - and re-decides on
                                  every resume
\`\`\`

Each checkpoint node captures its actor's input into the model and suspends; each following
run resumes one checkpoint further. The model is the workflow's durable memory - anything a
later step needs must be mapped into "model.*" before the checkpoint. The manager's decision
lands at a graph.math decision node on the order checkpoint's continuation with THREE
outcomes: an approved decision routes to the next suspension point, an explicit rejection
routes to a terminal node that reports the manager's reason (the workflow ends), and anything
else - a missing or unrecognized decision - returns "suspend" to pause again, so an invalid
request can never end a long-running workflow by accident: the decision re-executes on the
next resume and re-evaluates whatever arrives.

Create the graph model
----------------------
Create the root node:

\`\`\`
create node root
with properties
purpose=Purchase workflow with three human checkpoints
name=tutorial-14
\`\`\`

Create the resume node. A resumed run jumps past its last checkpoint; a fresh transaction
(no record - never suspended, or expired) continues along the forward path into the
"check-fresh" validation gate with "model.run" set to "fresh":

\`\`\`
create node resume
with type Resume
with properties
purpose=Restore workflow state if this transaction was suspended earlier
skill=graph.resume
task=v1.redis.retrieve.model
\`\`\`

Create the input validation gate. The variable substitution inside the text() constant is
null-safe: when the request has no "item" field it is not an order submission, so a later-stage
request without a suspended record is rejected:

\`\`\`
create node check-fresh
with type Decision
with properties
purpose=A fresh transaction must be an order submission
skill=graph.math
statement[]=MAPPING: text(={input.body.item}) -> model.order_probe
statement[]='''
IF: {model.order_probe} == '=null'
THEN: reject
ELSE: order
'''
\`\`\`

Create the three checkpoint nodes. Each captures its actor's input into the model and stages a
stage-specific reply for the caller (overriding the default suspended response). No property is
needed: the drawn edge to the suspend node (you will connect it below) IS the suspension
declaration. A checkpoint node is a complete working node - it executes its skill in full and
may carry any non-routing skill (graph.data.mapper here; graph.task, graph.api.fetcher and
graph.extension work the same way) - only its exit changes. The "Suspensible" type is purely
visual - it picks the node color in the Playground:

\`\`\`
create node order
with type Suspensible
with properties
purpose=Capture the customer order, then suspend for the store manager
skill=graph.data.mapper
mapping[]=input.body -> model.order
mapping[]=text(order-submitted; waiting for store manager approval) -> output.body.stage
mapping[]=model.run -> output.body.run
mapping[]=model.cid -> output.body.cid
\`\`\`

\`\`\`
create node approval
with type Suspensible
with properties
purpose=Capture the store manager approval, then suspend for the delivery department
skill=graph.data.mapper
mapping[]=input.body -> model.approval
mapping[]=text(approved; waiting for the delivery department to release the shipment) -> output.body.stage
mapping[]=model.run -> output.body.run
mapping[]=model.cid -> output.body.cid
\`\`\`

\`\`\`
create node delivery
with type Suspensible
with properties
purpose=Capture the shipment release, then suspend for shipment confirmation
skill=graph.data.mapper
mapping[]=input.body -> model.delivery
mapping[]=text(released; waiting for shipment confirmation) -> output.body.stage
mapping[]=model.run -> output.body.run
mapping[]=model.cid -> output.body.cid
\`\`\`

Create the manager decision. It sits on the order checkpoint's continuation, so every resumed
run lands here with the manager's input. Three outcomes: "approved" continues to the approval
checkpoint, "rejected" ends the workflow with the manager's reason, and anything else RETURNS
"suspend" to pause - the decision pattern. A pausing decision draws NO edge to the suspend node
(its drawn edges are outcome alternatives, and the gate rejects a decision-to-suspend edge); it
is re-executed against the new request input on every resume, so the workflow simply keeps
waiting until an explicit "approved" or "rejected" arrives - a wait loop with no extra nodes.
The probe reuses the same null-safe idiom as "check-fresh". The awaiting reply is staged
unconditionally before the IFs; the approval and rejection paths overwrite it downstream:

\`\`\`
create node check-approval
with type Decision
with properties
purpose=Approved continues, rejected ends the workflow, anything else keeps waiting
skill=graph.math
statement[]=MAPPING: text(={input.body.decision}) -> model.approval_probe
statement[]=MAPPING: text(awaiting-decision; supply decision approved or rejected for the store manager) -> output.body.stage
statement[]=MAPPING: model.run -> output.body.run
statement[]=MAPPING: model.cid -> output.body.cid
statement[]='''
IF: {model.approval_probe} == '=approved'
THEN: approval
ELSE: next
'''
statement[]='''
IF: {model.approval_probe} == '=rejected'
THEN: manager-reject
ELSE: suspend
'''
\`\`\`

\`\`\`
create node manager-reject
with type mapper
with properties
purpose=The manager rejected the purchase: report the reason and end the workflow
skill=graph.data.mapper
mapping[]=text(rejected) -> output.body.stage
mapping[]=input.body.reason -> output.body.reason
mapping[]=model.order -> output.body.order
mapping[]=model.run -> output.body.run
mapping[]=model.cid -> output.body.cid
\`\`\`

Create the completion, rejection, suspend and end nodes:

\`\`\`
create node ship
with type mapper
with properties
purpose=Ship to the customer with the full order history
skill=graph.data.mapper
mapping[]=text(shipped) -> output.body.stage
mapping[]=model.run -> output.body.run
mapping[]=model.order -> output.body.order
mapping[]=model.approval -> output.body.approval
mapping[]=model.delivery -> output.body.delivery
mapping[]=input.body -> output.body.shipment
mapping[]=model.cid -> output.body.cid
\`\`\`

\`\`\`
create node reject
with type mapper
with properties
purpose=Reject a request that has no suspended transaction and is not an order
skill=graph.data.mapper
mapping[]=int(404) -> output.status
mapping[]=text(rejected) -> output.body.type
mapping[]=text(Transaction not found. Submit the order first) -> output.body.message
mapping[]=model.run -> output.body.run
\`\`\`

\`\`\`
create node suspend
with type Suspend
with properties
purpose=Persist workflow state to Redis and wait for the next actor
skill=graph.suspend
task=v1.redis.persist.model
ttl=1h
\`\`\`

\`\`\`
create node end
\`\`\`

Connect the nodes. Every checkpoint node draws BOTH edges - the checkpoint edge to "suspend"
(the suspension declaration) and the continuation edge to the next step (where a resumed run
continues). The check-approval decision draws only its outcome edges - its waiting path is
the jump inside the IF-THEN-ELSE:

\`\`\`
connect root to resume with then
connect resume to check-fresh with fresh
connect check-fresh to order with submission
connect check-fresh to reject with no-transaction
connect order to suspend with checkpoint
connect order to check-approval with next
connect check-approval to approval with approved
connect check-approval to manager-reject with rejected
connect manager-reject to end with then
connect approval to suspend with checkpoint
connect approval to delivery with next
connect delivery to suspend with checkpoint
connect delivery to ship with next
connect ship to end with then
connect reject to end with then
connect suspend to end with then
\`\`\`

For your convenience, this graph model is preloaded as "tutorial-14".

Dry-run the workflow interactively
----------------------------------
You can exercise all three checkpoints without leaving the playground. Two things to
remember: instantiate before every run so each round starts with a fresh state machine
('run' may repeat on one instance and model values persist across runs -
see 'help run' - which would pollute a short-run simulation); and the SAME model.cid
must be supplied each time - it is the resume key. Redis must be running.

Import the deployed model as a draft:

\`\`\`
import graph from tutorial-14
\`\`\`

Run 1 - the customer orders a laptop:

\`\`\`
instantiate graph
text(order-1001) -> model.cid
text(laptop) -> input.body.item
int(2000) -> input.body.amount
\`\`\`

\`\`\`
run
\`\`\`

The traversal walks root -> resume -> check-fresh -> order -> suspend -> end and the run
completes normally - the workflow state now lives in Redis, not in memory. Inspect the
staged reply:

\`\`\`
inspect output.body
\`\`\`

It shows stage=order-submitted..., run=fresh (a new transaction) and cid=order-1001.

Run 2 - the store manager approves. Instantiate again with the same correlation ID and
the manager's input:

\`\`\`
instantiate graph
text(order-1001) -> model.cid
text(approved) -> input.body.decision
text(store-88) -> input.body.manager
\`\`\`

\`\`\`
run
\`\`\`

Watch the console: the resume node restores the persisted state and the traversal
continues at the check-approval decision - the order checkpoint is NOT re-executed. The
approved decision routes to the approval checkpoint. Now "inspect output.body" shows
stage=approved... and run=resume, and the "seen" command lists the order node as visited
even though this run never executed it - that is the restored traversal bookkeeping.

(The manager could reject instead: the same run with
"text(rejected) -> input.body.decision" and "text(budget exceeded) -> input.body.reason"
routes to manager-reject - the reply carries stage=rejected with the reason and the
original order, and the workflow ends. And if the run carries no valid decision at all -
including a replay against a leftover suspended record from an earlier exercise - the
workflow does NOT end: it replies stage=awaiting-decision and re-suspends, waiting for a
proper "approved" or "rejected". You will try both over REST below. Tip: records are
consumed on resume and re-created on each suspension, so if you repeat these exercises,
use a fresh correlation ID for each clean start.)

Run 3 - the delivery department releases the shipment:

\`\`\`
instantiate graph
text(order-1001) -> model.cid
boolean(true) -> input.body.release
text(express) -> input.body.courier
\`\`\`

\`\`\`
run
\`\`\`

Run 4 - shipment confirmation completes the workflow:

\`\`\`
instantiate graph
text(order-1001) -> model.cid
text(TRK-12345) -> input.body.tracking
\`\`\`

\`\`\`
run
\`\`\`

Inspect the final reply - the model accumulated state across all four short runs:

\`\`\`
inspect output.body
\`\`\`

It shows stage=shipped, run=resume, and the full history: order (laptop/2000), approval
(approved/store-88), delivery (release/express) and shipment (TRK-12345).

To see the input validation, start over with a correlation ID that never ordered:

\`\`\`
instantiate graph
text(order-9999) -> model.cid
text(approved) -> input.body.decision
\`\`\`

\`\`\`
run
\`\`\`

"inspect output" shows status=404 with type=rejected and run=fresh - the order must come
first, and the run flag tells the caller why. Each record is consumed on resume, so
repeating any middle run behaves the same way: no record means a fresh transaction.

Test the workflow over REST
---------------------------
Run 1 - the customer orders a laptop:

\`\`\`
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-14 \\
  -H "Content-Type: application/json" \\
  -H "X-Correlation-Id: order-1001" \\
  -d '{"item": "laptop", "amount": 2000}'
\`\`\`

The reply is {"stage": "order-submitted; waiting for store manager approval", "run": "fresh",
"cid": "order-1001"} and the run is over - nothing stays in memory. Every stage reply carries
the "run" flag ("fresh" on run 1, "resume" on runs 2 to 4) so the caller always knows whether
it is looking at a new transaction or a resumed continuation. Run 2 - the store manager approves:

\`\`\`
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-14 \\
  -H "Content-Type: application/json" \\
  -H "X-Correlation-Id: order-1001" \\
  -d '{"decision": "approved", "manager": "store-88"}'
\`\`\`

Run 3 - the delivery department releases the shipment:

\`\`\`
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-14 \\
  -H "Content-Type: application/json" \\
  -H "X-Correlation-Id: order-1001" \\
  -d '{"release": true, "courier": "express"}'
\`\`\`

Run 4 - shipment confirmation completes the workflow:

\`\`\`
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-14 \\
  -H "Content-Type: application/json" \\
  -H "X-Correlation-Id: order-1001" \\
  -d '{"tracking": "TRK-12345"}'
\`\`\`

The final reply carries the whole history - the order from run 1, the approval from run 2, the
release from run 3 and the shipment from run 4 - proof that the workflow state crossed every
suspension. Now try a decision with a correlation ID that never ordered:

\`\`\`
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-14 \\
  -H "Content-Type: application/json" \\
  -H "X-Correlation-Id: order-9999" \\
  -d '{"decision": "approved"}'
\`\`\`

The workflow rejects it with HTTP-404 - the order must come first - and the reply's
"run": "fresh" tells the UI why: the record expired or never existed. Each record is consumed on
resume, so a duplicated request at any stage behaves like a fresh transaction instead of
executing that stage twice.

Finally, try the manager's other option - reject with a reason. Submit a new order, then
reject it:

\`\`\`
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-14 \\
  -H "Content-Type: application/json" \\
  -H "X-Correlation-Id: order-2002" \\
  -d '{"item": "monitor", "amount": 300}'
\`\`\`

\`\`\`
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-14 \\
  -H "Content-Type: application/json" \\
  -H "X-Correlation-Id: order-2002" \\
  -d '{"decision": "rejected", "reason": "budget exceeded"}'
\`\`\`

The reply is {"stage": "rejected", "reason": "budget exceeded", "order": {...}, "run": "resume",
"cid": "order-2002"} and the workflow is over - the record was consumed on resume and nothing
re-suspended, so any further request under order-2002 is a fresh 404 rejection.

An invalid or missing decision behaves differently - the workflow stays alive. Submit another
order under order-3003, then send a request with no decision:

\`\`\`
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-14 \\
  -H "Content-Type: application/json" \\
  -H "X-Correlation-Id: order-3003" \\
  -d '{"note": "no decision here"}'
\`\`\`

The reply is {"stage": "awaiting-decision; supply decision approved or rejected for the store
manager", "run": "resume", "cid": "order-3003"} and the workflow re-suspended - repeat with
{"decision": "approved"} and it continues to the delivery stage as usual. Only an explicit
"approved" or "rejected" moves the workflow forward.

Summary
-------
In this session, we expressed a purchase workflow with three human checkpoints as four short
graph runs keyed by one business correlation ID: one reserved "suspend" node served every
checkpoint, each checkpoint node declared its suspension with a drawn edge, captured its
actor's input into the model and staged its own stage response, a graph.math decision at the
manager's resumption point routed an approval to the next checkpoint, a rejection (with the
manager's reason) to the end, and anything else returned "suspend" to pause again (a pausing
decision re-executes and re-decides on every resume, so no extra wait nodes are needed),
input validation enforced the order-before-decision sequence, and the engine-managed
"model.run" flag told every reply whether the run was fresh or resumed.

Why suspend and resume?
-----------------------
Real business processes wait on people - repeatedly. Suspension turns each wait into a durable
record instead of a parked runtime: any application instance sharing the state store can resume
the workflow, restarts lose nothing, and each run stays short and observable. The state store is
pluggable - Redis is the packaged implementation, and any composable function honoring the
documented store contract can replace it.
`,At=`Tutorial 2
----------
In this tutorial, you will deploy the 'hello world' graph model that you created in
tutorial 1, then enhance it into an echo application.

Exercise
--------
To deploy the graph model from tutorial 1, copy the 'tutorial-1.json' file that was
exported earlier into your application's resources/graph folder
(src/main/resources/graph in a Java project, resources/graph in a Rust project).

\`\`\`
cp /tmp/graph/tutorial-1.json ~/sandbox/{your_project}/src/main/resources/graph
\`\`\`

The temp graph folder and the graph manifest are set in the application configuration
file (application.properties or application.yml):

\`\`\`properties
#
# temp graph working location
# (temp graph location must use "file:/" prefix because of READ/WRITE requirements)
#
location.graph.temp=file:/tmp/graph
#
# the graph manifest - the quality gate and the only door to deployed execution
#
graph.model.automation=classpath:/graphs.yaml
\`\`\`

The deployed graph folder is declared in the graph manifest itself - like flows.yaml, the
manifest carries the location of its own models. Add your graph ID to the manifest so the
CompileGraph quality gate validates it at startup; only graphs that pass become executable:

\`\`\`yaml
graphs:
  - 'tutorial-1'

location: 'classpath:/graph'
\`\`\`

The 'location' entry is optional (default 'classpath:/graph'; a read-only folder, so
'file:/' or 'classpath:/' both work).

Invoke the graph API REST endpoint
----------------------------------
The generic graph API endpoint is \`POST /api/graph/{graph_id}\`, where 'graph_id' is
the name of the graph model.

To make a request to the 'tutorial-1' graph model, enter the following curl command.

\`\`\`
> curl -X POST http://127.0.0.1:8085/api/graph/tutorial-1
hello world
\`\`\`

It returns 'hello world'.

Since the "hello world" graph model does not require any input parameter, you can also
use HTTP GET to execute the graph.

\`\`\`
> curl http://127.0.0.1:8085/api/graph/tutorial-1
hello world
\`\`\`

In the application log, you will see the 'telemetry' of the event flow. The HTTP POST
request is received by the 'http.flow.adapter' that executes a flow called
'graph-executor'.

The Graph Executor creates an instance of the graph, traverses from the "root" node
and comes to the "end" node that contains the "graph.data.mapper" skill. The data
mapper sets the output to "hello world", which is routed to "async.http.response"
and returned to the curl command.

The telemetry entries look like this (abridged; the Rust engine logs the same trace
records in its own JSON log format):

\`\`\`
2026-03-31 15:19:08.052 INFO  Telemetry:81 - {trace={path=POST /api/graph/tutorial-1,
    service=http.flow.adapter, success=true, from=http.request, exec_time=0.12, status=200}}
2026-03-31 15:19:08.055 INFO  Telemetry:81 - {trace={path=POST /api/graph/tutorial-1,
    service=graph.data.mapper, success=true, from=graph.executor, exec_time=0.074, status=200},
    annotations={node=end}}
2026-03-31 15:19:08.056 INFO  GraphHousekeeper:44 - Graph instance 2c1a00d63f7d4ec2b657db4a75021068
    for model 'tutorial-1' cleared
2026-03-31 15:19:08.056 INFO  Telemetry:81 - {trace={path=POST /api/graph/tutorial-1,
    service=task.executor, success=true, from=event.script.manager, exec_time=4.0, status=200},
    annotations={execution=Run 1 task in 4 ms, flow=graph-executor}}
2026-03-31 15:19:08.056 INFO  Telemetry:81 - {trace={path=POST /api/graph/tutorial-1,
    service=async.http.response, success=true, from=task.executor, exec_time=0.224, status=200}}
\`\`\`

Let's enhance the graph model to echo input.

Import the graph model
----------------------
You can import the tutorial-1 graph model like this:

\`\`\`
> import graph from tutorial-1
Graph model imported as draft
\`\`\`

The graph diagram is shown in the right panel under the "Graph" tab.

Edit the nodes
--------------
Enter an "edit node" command to print out the root node content.

\`\`\`
> edit node root
update node root
with type Root
with properties
name=tutorial-1
purpose=Tutorial one to return a 'hello world' message
\`\`\`

Copy-and-paste the "update node" block into the input box and modify it as:

\`\`\`
update node root
with type Root
with properties
name=tutorial-2
purpose=Tutorial two to echo a user message
\`\`\`

Press enter and you will see:

\`\`\`
> update node root...
node root updated
\`\`\`

Then update the end node in the same fashion. Modify its content like this:

\`\`\`
update node end
with type End
with properties
mapping[]=input.body -> output.body
skill=graph.data.mapper
\`\`\`

Perform a dry-run
-----------------
To run the updated graph model, use the \`instantiate graph\` command with some
mock input content.

\`\`\`
> instantiate graph
  text(it works) -> input.body.message
Graph instance created. Loaded 1 mock entry, model.ttl = 30000 ms
\`\`\`

In the above command, you insert the constant value "it works" into the "message"
key of the "input.body" namespace.

Enter "run" to do a dry-run and you will see this:

\`\`\`
> run
Walk to root
Walk to end
Executed end with skill graph.data.mapper in 0.43 ms
{
  "output": {
    "body": {
      "message": "it works"
    }
  }
}
Graph traversal completed in 2 ms
\`\`\`

Export the updated graph model
------------------------------
You may export the updated graph model as "tutorial-2".

\`\`\`
> export graph as tutorial-2
Graph exported to /tmp/graph/tutorial-2.json
Described in /api/graph/model/tutorial-2/235-7
\`\`\`

Deploy the graph model
----------------------
Repeat the deployment step at the beginning of this tutorial: copy
"/tmp/graph/tutorial-2.json" into your application's resources/graph folder.

Test the deployed graph model
-----------------------------
Restart your application to load the deployed graphs into memory.

Send the following curl command:

\`\`\`
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-2 \\
  -H "Content-Type: application/json" \\
  -d '{
    "greeting": "Hello",
    "message": "it is a wonderful day"
  }'
\`\`\`

It responds with:

\`\`\`json
{
  "greeting": "Hello",
  "message": "it is a wonderful day"
}
\`\`\`

Summary
-------
In this tutorial, you have completed the following exercise:

1. deployed the graph model 'tutorial-1' and invoked the API that executes the graph model as an instance
2. enhanced the graph model from a simple 'hello world' application to an echo program
3. performed a dry-run with mock input to test the response
4. exported the updated graph model as 'tutorial-2'
5. deployed the 'tutorial-2' graph model
6. tested the 'tutorial-2' graph model using an HTTP POST request with an input payload
`,jt=`Tutorial 3
----------
In this tutorial, you will learn the data dictionary method to source data from an
external service.

Exercise
--------
You will create a root node, an end node, two data dictionary nodes, a data provider
node and an API fetcher node.

To clear the previous graph session, click the Tools button in the top-right corner
and click the "Stop" and "Start" toggle button. A new graph session will start.

Create root and end nodes
-------------------------
Enter the "create node" command for the "root" and "end" nodes first.

\`\`\`
create node root
with type Root
with properties
name=tutorial-3
purpose=Demonstrate data sourcing using the Data Dictionary method - fetch one person profile (name and address) by person_id
\`\`\`

\`\`\`
create node end
with type End
\`\`\`

Create data dictionary items
----------------------------
A data dictionary describes a "data attribute" and its "data provider". Please enter
the following:

\`\`\`
create node person-name
with type Dictionary
with properties
purpose=name of a person
provider=mdm-profile
input[]=person_id
output[]=response.profile.name -> result.name

create node person-address
with type Dictionary
with properties
purpose=address of a person
provider=mdm-profile
input[]=person_id
output[]=response.profile.address -> result.address
\`\`\`

This creates two Dictionary nodes, "person-name" and "person-address", both served by
a data provider called "mdm-profile".

In a Dictionary node, \`input[]\` entries are **bare parameter names** — not
\`source -> target\` mappings. Here, the parameter required to retrieve these data
attributes is "person_id". If a parameter has a sensible default, supply it with an
optional \`:{default}\` suffix (e.g. \`input[]=detail:true\`) — a default value is the
only meaning of \`:\` in a Dictionary input entry.

The \`output[]\` section maps the provider's response into the dictionary's result set.
The \`response.\` and \`result.\` namespaces represent the response key-values from the
data provider and the result key-values produced by this data dictionary.

In the "person-name" data dictionary, the output mapping extracts the "profile.name"
attribute from the response's data structure and exposes it as the key "name".

Create a data provider
----------------------
The data dictionaries name a data provider "mdm-profile". Create a node for it:

\`\`\`
create node mdm-profile
with type Provider
with properties
purpose=Master Data Management's profile management endpoint
url=http://127.0.0.1:\${rest.server.port:8080}/api/mdm/profile/{id}
method=GET
feature[]=log-request-headers
feature[]=log-response-headers
input[]=text(application/json) -> header.accept
input[]=person_id -> path_parameter.id
\`\`\`

The "url" is the REST endpoint of the target service. \`\${rest.server.port:8080}\`
resolves a key-value from the application configuration or an environment variable;
the value after the optional \`:\` is a default.

In this example, the url has a path parameter "id" — filled by the \`input[]\` line
that targets \`path_parameter.id\`.

The "feature" section tells the system to apply pre-processing and/or post-processing
to the HTTP request/response. "log-request-headers" logs the request headers, if any,
and "log-response-headers" logs the HTTP response headers from the target service.
These two features are for demonstration; in a real-world use case, you might
implement an "oauth2-bearer" feature. Custom features are discussed in a subsequent
tutorial.

The input section maps values into the outgoing HTTP request — headers, path
parameters, query and/or body key-values. The target namespaces are:

\`\`\`
header.
query.
path_parameter.
body.
\`\`\`

The left-hand side of a provider input mapping is a constant (e.g.
\`text(application/json)\`) or an input parameter declared by the associated data
dictionary (e.g. \`person_id\`).

Create an API fetcher
---------------------
Create a fetcher node like this:

\`\`\`
create node fetcher
with type Fetcher
with properties
skill=graph.api.fetcher
dictionary[]=person-name
dictionary[]=person-address
input[]=input.body.person_id -> person_id
output[]=result.name -> output.body.name
output[]=result.address -> output.body.address
\`\`\`

After this step, you will see 6 nodes in the graph diagram on the right panel.

Connect the fetcher
-------------------
Connect the root node to the fetcher node, then the fetcher to the end node.

\`\`\`
> connect root to fetcher with fetch
node root connected to fetcher
> connect fetcher to end with complete
node fetcher connected to end
\`\`\`

Export the graph model
----------------------
The execution path is complete. Let's export it as 'tutorial-3'.

\`\`\`
> export graph as tutorial-3
Graph exported to /tmp/graph/tutorial-3.json
Described in /api/graph/model/tutorial-3/849-13
\`\`\`

Test the fetcher node
---------------------
Before you do a dry-run, you can test the fetcher alone because it is self-contained:
it maps the input parameter to 'person_id', makes an outgoing HTTP request using the
data dictionary and returns the result as "output.body".

First, instantiate the graph model and mock the input parameter like this:

\`\`\`
instantiate graph
int(100) -> input.body.person_id
\`\`\`

The system acknowledges your command as follows:

\`\`\`
> instantiate graph...
Graph instance created. Loaded 1 mock entry, model.ttl = 30000 ms
\`\`\`

Before you test the fetcher, check the input and output key-values with the
\`inspect\` command:

\`\`\`
> inspect input
{
  "inspect": "input",
  "outcome": {
    "body": {
      "person_id": 100
    }
  }
}
> inspect output
{
  "inspect": "output",
  "outcome": {}
}
\`\`\`

When a graph model is instantiated, the system creates a temporary "state machine"
for the graph instance. The inspect command lets you check the current key-values in
that state machine.

The above output shows that "person_id" with the integer value 100 is stored in
input.body, and there is nothing in the output yet.

You can now test the fetcher with the "execute" command:

\`\`\`
> execute fetcher
node fetcher run for 0.266 ms with exit path 'next'
\`\`\`

The fetcher has been executed and it is ready to continue to the next node.

Now inspect the "output" in the state machine again.

\`\`\`
> inspect output
{
  "inspect": "output",
  "outcome": {
    "body": {
      "address": "100 World Blvd",
      "name": "Peter"
    }
  }
}
\`\`\`

The result set contains the name and address obtained from the target service.

Dry-run
-------
The fetcher is configured correctly, so you can do a dry-run from beginning to end.

Clear the state machine by instantiating the graph model again:

\`\`\`
instantiate graph
int(100) -> input.body.person_id
\`\`\`

\`\`\`
> instantiate graph...
Graph instance created. Loaded 1 mock entry, model.ttl = 30000 ms
\`\`\`

Verify that the output key-values are cleared with \`inspect output\`. Then enter \`run\`.

\`\`\`
> run
Walk to root
Walk to fetcher
Executed fetcher with skill graph.api.fetcher in 14.456 ms
Walk to end
{
  "output": {
    "body": {
      "address": "100 World Blvd",
      "name": "Peter"
    }
  }
}
Graph traversal completed in 15 ms
\`\`\`

List nodes and connections
--------------------------
Let's check the nodes and connections of the graph model 'tutorial-3'.

Enter the \`list nodes\` and \`list connections\` commands:

\`\`\`
> list nodes
root [Root]
fetcher [Fetcher]
mdm-profile [Provider]
person-address [Dictionary]
person-name [Dictionary]
end [End]
> list connections
root -[fetch]-> fetcher
fetcher -[complete]-> end
\`\`\`

Note that the data dictionary and data provider nodes have no connections yet. They
are "configuration" nodes — not active nodes that execute on their own. The API
fetcher references them by name and uses their configuration to make an external
API call.

For more details of the data dictionary method, enter "help data-dictionary".

Configuration nodes must still not be left floating — the next step wires them into
the graph's knowledge layer.

Create an island to hold the data dictionary
--------------------------------------------
The required convention is: **leave no node unconnected**. Configuration nodes are
wired into the graph's knowledge layer with an "island" node.

\`\`\`
create node dictionary
with type Island
with properties
skill=graph.island
\`\`\`

Then connect the data dictionary nodes and the provider node to it.

\`\`\`
> connect root to dictionary with contains
node root connected to dictionary
> connect dictionary to person-name with data
node dictionary connected to person-name
> connect dictionary to person-address with data
node dictionary connected to person-address
> connect person-name to mdm-profile with provider
node person-name connected to mdm-profile
> connect person-address to mdm-profile with provider
node person-address connected to mdm-profile
> list connections
root -[contains]-> dictionary
root -[fetch]-> fetcher
dictionary -[data]-> person-address
dictionary -[data]-> person-name
person-address -[provider]-> mdm-profile
person-name -[provider]-> mdm-profile
fetcher -[complete]-> end
\`\`\`

A "graph.island" node is isolated from graph traversal: it never hands execution to
the next node, so the execution path is unaffected. Its purpose is knowledge
structure — the island subgraph is the graph's entity-relationship diagram.

Data entities such as person, account and order, and the directional relationships
between them, represent enterprise knowledge. With the dictionaries, providers and
entities wired under the island, the graph becomes living documentation: a new team
member (or an AI agent) can read the knowledge layer to discover the domain model,
not just the execution path.

To save the updated graph model, export it again.

\`\`\`
> export graph as tutorial-3
Graph exported to /tmp/graph/tutorial-3.json
Described in /api/graph/model/tutorial-3/287-4
\`\`\`

Deploy the graph model
----------------------
To deploy, copy "/tmp/graph/tutorial-3.json" into your application's resources/graph
folder and restart the application. You can then invoke the knowledge graph endpoint
with the following curl command.

\`\`\`
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-3 \\
  -H "Content-Type: application/json" \\
  -d '{
    "person_id": 100
  }'
\`\`\`

Note that input parameters, if any, must be submitted as a POST request body with
content type "application/json".

You will receive the following response:

\`\`\`json
{
  "address": "100 World Blvd",
  "name": "Peter"
}
\`\`\`

If you change the person_id to 10, you will receive an error because the test profile
is set to 100.

\`\`\`json
{
  "message": "Profile 10 not found",
  "type": "error",
  "target": "person-name",
  "status": 400
}
\`\`\`

Well done! You have successfully created a graph model that fetches external data.

API call optimization
---------------------
If you check the application log, you will notice that each graph instance makes only
one HTTP call to \`http://127.0.0.1:8085/api/mdm/profile/10\`.

When multiple data dictionary items share the same target URL, method and input
parameter values, the system avoids making redundant API calls.

Therefore, it is important to configure the data dictionary and provider correctly so
that the system fetches data efficiently.

Summary
-------
In this tutorial, you configured a data dictionary and a data provider, and defined an
API fetcher node that uses them to fetch data. You deployed the graph model and made
an API request with a curl command.

You also organized the data dictionary and provider nodes under an "island" — the
required knowledge-layer convention that leaves no node unconnected.
`,Mt=`Tutorial 4
----------
In this tutorial, you will set up simple mathematics and boolean operations in a
graph model to make a decision.

Exercise
--------
You will create a root node, an end node and a decision node.

To clear the previous graph session, click the Tools button in the top-right corner
and click the "Stop" and "Start" toggle button. A new graph session will start.

Create root and end nodes
-------------------------
Enter the "create node" command for the "root" and "end" nodes first.

\`\`\`
create node root
with type Root
with properties
name=tutorial-4
purpose=Demonstrate decision making using mathematics and boolean operations
\`\`\`

Assume there are two input parameters (a and b). The 'decision' node will add the two
numbers, and the end node will echo the input parameters and the sum.

\`\`\`
create node end
with type End
with properties
skill=graph.data.mapper
mapping[]=input.body.a -> output.body.a
mapping[]=input.body.b -> output.body.b
mapping[]=decision.result.c -> output.body.sum
\`\`\`

Create a decision node
----------------------
Create a node with the skill 'graph.math' to do decision-making.

\`\`\`
create node decision
with type Decision
with properties
skill=graph.math
statement[]=COMPUTE: c -> {input.body.a} + {input.body.b}
statement[]='''
IF: {input.body.a} >= {input.body.b}
THEN: next
ELSE: less-than
'''
statement[]=MAPPING: text(a >= b) -> output.body.message
statement[]=MAPPING: boolean(false) -> output.body.less_than
\`\`\`

The skill "graph.math" supports these statement types:

| Type         | Operation                                                       |
|--------------|-----------------------------------------------------------------|
| COMPUTE      | generate a value (LHS) from a mathematics expression (RHS)      |
| IF-THEN-ELSE | evaluate a boolean condition and select the next node           |
| MAPPING      | perform a data mapping operation                                |
| EXECUTE      | run another graph.math node's statements inline (module reuse)  |
| RESET        | reset the current state of one or more nodes                    |

The 'RESET' and 'EXECUTE' features are covered in more advanced tutorials.
Enter "help graph-math" for the full statement grammar.

Use the 'triple single quote' syntax to enter the IF-THEN-ELSE statement as one
multi-line value.

The IF line is a boolean expression. THEN names the next step when the expression is
true; ELSE names the next step when it is false. Each may be a node name or the
keyword 'next'.

Statements are evaluated in order. A branch that resolves to 'next' falls through to
the statements after the IF-THEN-ELSE — in this example, the two MAPPING statements
that set the positive-case output key-values. A branch that jumps to a named node
(here 'less-than') ends the statement list immediately, so those mappings do not run.

The curly brace syntax \`{key}\` substitutes the value of the bracketed key inside a
COMPUTE or IF expression. A MAPPING statement does not use curly braces — it is data
mapping only, where the left-hand side is a constant, an input parameter or a model
variable, and the right-hand side is a model or output variable.

Create a node to handle the negative case
-----------------------------------------
Create a node called "less-than" to handle the negative case from the decision node.

\`\`\`
create node less-than
with type Reject
with properties
mapping[]=text(a < b) -> output.body.message
mapping[]=boolean(true) -> output.body.less_than
skill=graph.data.mapper
\`\`\`

Connect the nodes
-----------------

\`\`\`
connect root to decision with evaluate
connect less-than to end with negative
connect decision to end with positive
\`\`\`

The "less-than" node is reached only when the decision node evaluates "a < b", so it
does not need a connection from the root. When it finishes, it hands off to the "end"
node. A "list connections" command shows:

\`\`\`
> list connections
root -[evaluate]-> decision
decision -[positive]-> end
less-than -[negative]-> end
\`\`\`

You can also use the "describe node" command to see a node's content and connections:

\`\`\`
> describe node decision
{
  "node": {
    "types": [
      "Decision"
    ],
    "alias": "decision",
    "id": "c9b30d7d8a6c4d49a88b5a9254fe44e2",
    "properties": {
      "skill": "graph.math",
      "statement": [
        "COMPUTE: c -> {input.body.a} + {input.body.b}",
        "IF: {input.body.a} >= {input.body.b}
         THEN: next
         ELSE: less-than",
        "MAPPING: text(a >= b) -> output.body.message",
        "MAPPING: boolean(false) -> output.body.less_than"
      ]
    }
  },
  "from": [
    "root"
  ],
  "to": [
    "end"
  ]
}
\`\`\`

Test the positive case
----------------------
To test the positive case, mock the input values and instantiate the graph model.
Note that "start" is an alias of "instantiate".

\`\`\`
start graph
int(100) -> input.body.a
int(50) -> input.body.b
\`\`\`

Then test the graph model with the "run" command:

\`\`\`
> run
Walk to root
Walk to decision
Executed decision with skill graph.math in 0.824 ms
Walk to end
Executed end with skill graph.data.mapper in 0.099 ms
{
  "output": {
    "body": {
      "a": 100,
      "b": 50,
      "less_than": false,
      "sum": 150.0,
      "message": "a >= b"
    }
  }
}
Graph traversal completed in 7 ms
\`\`\`

Note that "sum" is 150.0 — a COMPUTE statement evaluates to a floating-point number,
so an integer result serializes with a decimal point (it is numerically exact).

Test the negative case
----------------------

\`\`\`
start graph
int(180) -> input.body.a
int(250) -> input.body.b
\`\`\`

When you do a dry-run, it shows the following:

\`\`\`
> run
Walk to root
Walk to decision
Executed decision with skill graph.math in 0.394 ms
Walk to less-than
Executed less-than with skill graph.data.mapper in 0.054 ms
Walk to end
Executed end with skill graph.data.mapper in 0.051 ms
{
  "output": {
    "body": {
      "a": 180,
      "b": 250,
      "less_than": true,
      "sum": 430.0,
      "message": "a < b"
    }
  }
}
Graph traversal completed in 2 ms
\`\`\`

Export the graph model
----------------------
Save the graph model by exporting it.

\`\`\`
> export graph as tutorial-4
Graph exported to /tmp/graph/tutorial-4.json
Described in /api/graph/model/tutorial-4/804-24
\`\`\`

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-4.json" to your application's
resources/graph folder. You can then test the deployed model with a curl command.

Summary
-------
In this tutorial, you created a graph model that adds two numbers, compares them and
returns a decision.

While this is a trivial example, it demonstrates that you can build useful computation
and evaluation logic in an Active Knowledge Graph using just simple mathematics and
boolean operation statements.
`,Nt=`Tutorial 5
----------
In this tutorial, you will explore parallel processing and graph navigation using a
node with the skill 'graph.join'.

Exercise
--------
You will import the graph model from tutorial-3 and update it to fetch two user
profiles at the same time.

Import a graph model
--------------------
Enter 'import graph from tutorial-3'.

\`\`\`
> import graph from tutorial-3
Graph model not found in /tmp/graph/tutorial-3.json
Found deployed graph model in classpath:/graph
Please export an updated version and re-import to instantiate an instance model
\`\`\`

If you have not exported tutorial-3 earlier, the system imports it from a demo graph.

Examine the graph model
-----------------------
Examine the graph model with the 'list nodes' and 'list connections' commands.

\`\`\`
> list nodes
root [Root]
fetcher [Fetcher]
mdm-profile [Provider]
person-address [Dictionary]
person-name [Dictionary]
end [End]
> list connections
root -[fetch]-> fetcher
fetcher -[complete]-> end
\`\`\`

Review the fetcher node
-----------------------
Enter 'edit node fetcher' to review the configuration of the node. The system
displays the following:

\`\`\`
update node fetcher
with type Fetcher
with properties
dictionary[]=person-name
dictionary[]=person-address
input[]=input.body.person_id -> person_id
output[]=result.name -> output.body.name
output[]=result.address -> output.body.address
skill=graph.api.fetcher
\`\`\`

Create two new fetchers
-----------------------
Assume the use case is to fetch two user profiles at the same time. Create two
fetchers like this:

\`\`\`
create node fetcher-1
with type Fetcher
with properties
dictionary[]=person-name
dictionary[]=person-address
input[]=input.body.person1 -> person_id
output[]=result.name -> model.fetcher-1.name
output[]=result.address -> model.fetcher-1.address
output[]=model.fetcher-1 -> output.body.profile[]
skill=graph.api.fetcher
\`\`\`

\`\`\`
create node fetcher-2
with type Fetcher
with properties
dictionary[]=person-name
dictionary[]=person-address
input[]=input.body.person2 -> person_id
output[]=result.name -> model.fetcher-2.name
output[]=result.address -> model.fetcher-2.address
output[]=model.fetcher-2 -> output.body.profile[]
skill=graph.api.fetcher
\`\`\`

When two skilled nodes execute in parallel, pay attention to how they share the state
machine. Data mapping itself is thread-safe — state-machine operations are
serialized — but parallel branches must not write to the same scalar key: the last
writer wins, nondeterministically. Write to disjoint keys instead. Here, each fetcher
assembles its profile under its own temporary variable in the "model" namespace:
\`model.fetcher-1\` and \`model.fetcher-2\`.

The final output mapping uses the array append syntax \`[]\`, which appends the map
containing name and address to the 'profile' array. Appending with \`[]\` from parallel
branches is race-free, but the element order follows completion order — undetermined
across parallel branches. If you must guarantee that person1's result goes to array
element 0 and person2's to element 1, set the array element index directly:

\`\`\`
output[]=model.fetcher-1 -> output.body.profile[0]
\`\`\`

\`\`\`
output[]=model.fetcher-2 -> output.body.profile[1]
\`\`\`

Since profile order does not matter in this tutorial, we will use the append form \`[]\`.

Create a join node
------------------
Create a "join" node to synchronize the two parallel branches:

\`\`\`
create node join
with type Join
with properties
skill=graph.join
\`\`\`

Remove the original fetcher node
--------------------------------
Enter 'delete node fetcher' to remove the original fetcher node.

\`\`\`
> delete node fetcher
node fetcher deleted
\`\`\`

When the original fetcher is deleted, its connections to the root node and end node
are removed too.

Connect the new fetchers
------------------------
Enter the following to define the graph navigation.

\`\`\`
connect root to fetcher-1 with one
connect root to fetcher-2 with two
connect fetcher-1 to join with join
connect fetcher-2 to join with join
connect join to end with done
\`\`\`

Do a 'list connections' to confirm the setup.

\`\`\`
> list connections
root -[one]-> fetcher-1
root -[two]-> fetcher-2
fetcher-1 -[join]-> join
fetcher-2 -[join]-> join
join -[done]-> end
\`\`\`

Perform a dry-run
-----------------
Start the graph model with this mock input:

\`\`\`
start graph
int(100) -> input.body.person1
int(200) -> input.body.person2
\`\`\`

Then enter 'run' to execute the graph instance.

\`\`\`
> run
Walk to root
Walk to fetcher-2
Walk to fetcher-1
Executed fetcher-1 with skill graph.api.fetcher in 1.048 ms
Walk to join
Executed fetcher-2 with skill graph.api.fetcher in 0.931 ms
Walk to join
Executed join with skill graph.join in 0.04 ms
Walk to end
Executed join with skill graph.join in 0.017 ms
{
  "output": {
    "body": {
      "profile": [
        {
          "address": "100 World Blvd",
          "name": "Mary"
        },
        {
          "address": "100 World Blvd",
          "name": "Peter"
        }
      ]
    }
  }
}
Graph traversal completed in 6 ms
\`\`\`

If you check the application log, you will see the two fetchers executed in parallel.

\`\`\`
GET http://127.0.0.1:8085/api/mdm/profile/100, with [person_id], ttl=30000
GET http://127.0.0.1:8085/api/mdm/profile/200, with [person_id], ttl=30000
\`\`\`

Create an island to hold the data dictionary
--------------------------------------------
Just like tutorial 3, wire the data dictionary and provider nodes into the graph's
knowledge layer with an island node. This is the required convention — leave no node
unconnected: the island subgraph is the graph's entity-relationship diagram, turning
the graph into living documentation of enterprise knowledge.

\`\`\`
create node dictionary
with type Island
with properties
skill=graph.island
\`\`\`

Then connect the data dictionary nodes and the provider node to it.

\`\`\`
> connect root to dictionary with contains
node root connected to dictionary
> connect dictionary to person-name with data
node dictionary connected to person-name
> connect dictionary to person-address with data
node dictionary connected to person-address
> connect person-name to mdm-profile with provider
node person-name connected to mdm-profile
> connect person-address to mdm-profile with provider
node person-address connected to mdm-profile
> list connections
root -[contains]-> dictionary
root -[one]-> fetcher-1
root -[two]-> fetcher-2
dictionary -[data]-> person-address
dictionary -[data]-> person-name
fetcher-1 -[join]-> join
fetcher-2 -[join]-> join
person-address -[provider]-> mdm-profile
person-name -[provider]-> mdm-profile
join -[done]-> end
\`\`\`

Export the graph model
----------------------
Save the graph model by exporting it.

\`\`\`
> export graph as tutorial-5
Graph exported to /tmp/graph/tutorial-5.json
Described in /api/graph/model/tutorial-5/920-28
\`\`\`

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-5.json" to your application's
resources/graph folder. You can then test the deployed model with a curl command.

\`\`\`
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-5 \\
  -H "Content-Type: application/json" \\
  -d '{
    "person1": 100,
    "person2": 200
  }'
\`\`\`

Summary
-------
In this tutorial, you created a graph model capable of parallel processing. It makes
two API requests at the same time; the two branches then converge into a "join" node
before reaching the "end" node.

The execution of a graph instance is guided by graph traversal: it follows the
connections you define between nodes. If a node has a skill, the graph executor runs
the composable function that provides the skill; if not, the graph executor continues
to the next downstream node.
`,Pt=`Tutorial 6
----------
In this tutorial, you will create a graph model that fetches an array list from one
service and iterates over the elements of the array to fetch more details from
another service, using the "for_each" keyword.

Exercise
--------
You will import the graph model from tutorial-3 as a template and expand it to handle
a multi-step data fetch use case.

Import a graph model
--------------------
Enter 'import graph from tutorial-3'.

\`\`\`
> import graph from tutorial-3
Graph model not found in /tmp/graph/tutorial-3.json
Found deployed graph model in classpath:/graph
Please export an updated version and re-import to instantiate an instance model
\`\`\`

If you have not exported tutorial-3 earlier, the system imports it from a demo graph.

Examine the graph model
-----------------------
Examine the graph model with the 'list nodes' and 'list connections' commands.

\`\`\`
> list nodes
root [Root]
fetcher [Fetcher]
mdm-profile [Provider]
person-address [Dictionary]
person-name [Dictionary]
end [End]
> list connections
root -[fetch]-> fetcher
fetcher -[complete]-> end
\`\`\`

Create a new data dictionary node
---------------------------------
Enter the following to create a new data dictionary node "person-accounts". It uses
the same data provider "mdm-profile" to retrieve the list of accounts for a person —
an array of account numbers.

\`\`\`
create node person-accounts
with type Dictionary
with properties
input[]=person_id
output[]=response.accounts -> result.account_numbers
provider=mdm-profile
purpose=accounts of a person
\`\`\`

Update the fetcher
------------------
Add the dictionary item "person-accounts" to the original fetcher.

\`\`\`
update node fetcher
with type Fetcher
with properties
dictionary[]=person-name
dictionary[]=person-address
dictionary[]=person-accounts
input[]=input.body.person_id -> person_id
output[]=result.name -> output.body.name
output[]=result.address -> output.body.address
skill=graph.api.fetcher
\`\`\`

Create one more data dictionary node
------------------------------------
Create a data dictionary node "account-details", associated with the data provider
"account-details-provider", to retrieve account details based on person_id and
account_id.

\`\`\`
create node account-details
with type Dictionary
with properties
input[]=person_id
input[]=account_id
output[]=response.account.details -> result.accounts
provider=account-details-provider
purpose=Account details
\`\`\`

Create a new data provider
--------------------------
Enter the following to create the data provider that retrieves account details.

Its feature section declares oauth2-bearer, log-request-headers and
log-response-headers. The "oauth2-bearer" entry is a placeholder — implement it
according to your organization's security guidelines. Functionally, it would acquire
an OAuth2 bearer token from a security authority using a client id and secret
configured in the deployed environment, cache and refresh the access token as
required, and insert the "authorization" header in a pre-processing step of the Graph
API Fetcher. The log-request-headers and log-response-headers features can serve as
templates for implementing your own pre-processing and post-processing features.

\`\`\`
create node account-details-provider
with type Provider
with properties
feature[]=oauth2-bearer
feature[]=log-request-headers
feature[]=log-response-headers
input[]=text(application/json) -> header.accept
input[]=text(application/json) -> header.content-type
input[]=person_id -> body.person_id
input[]=account_id -> body.account_id
method=POST
purpose=Account Management Endpoint
url=http://127.0.0.1:\${rest.server.port}/api/account/details
\`\`\`

Note that this is a POST provider: the \`body.{key}\` input targets build the JSON
request body, and the parameters travel in the body rather than the URL.

Create a second fetcher
-----------------------
Create a second fetcher as follows. The \`for_each\` statement iterates over the array
in the first fetcher's result set (\`fetcher.result.account_numbers\`), mapping each
element into "model.account_number".

For each element, the input statement block runs to populate the input parameters:
"person_id" is passed unchanged to every call, while "account_id" takes the current
element.

\`\`\`
create node fetcher-2
with type Fetcher
with properties
dictionary[]=account-details
for_each[]=fetcher.result.account_numbers -> model.account_number
input[]=input.body.person_id -> person_id
input[]=model.account_number -> account_id
output[]=result.accounts -> output.body.accounts
skill=graph.api.fetcher
\`\`\`

Each iteration's \`result.accounts\` value is appended into a single array on this
node's result set — with five account numbers, "output.body.accounts" becomes an
array of five account detail records.

Rearrange the connections
-------------------------
Connect the first fetcher to the second fetcher, delete the original connection
between the fetcher and the end node, then connect the second fetcher to the end node.

Enter 'list connections' to show the updated connections.

\`\`\`
> connect fetcher to fetcher-2 with details
node fetcher connected to fetcher-2
> delete connection fetcher and end
fetcher -> end removed
> connect fetcher-2 to end with complete
node fetcher-2 connected to end
> list connections
root -[fetch]-> fetcher
fetcher -[details]-> fetcher-2
fetcher-2 -[complete]-> end
\`\`\`

Update the root node
--------------------
Since you are using the tutorial-3 graph model as a template, it is good practice to
update the root node to describe the new purpose of tutorial-6. Enter the following.

\`\`\`
update node root
with type Root
with properties
name=tutorial-6
purpose=Demonstrate multi-step API fetching and the "for_each" method
\`\`\`

Perform a dry-run
-----------------
Enter the following to mock the input parameter "person_id = 100".

\`\`\`
start graph
int(100) -> input.body.person_id
\`\`\`

Then enter \`run\` to do a dry-run. You will see the following:

\`\`\`
> start graph...
Graph instance created. Loaded 1 mock entry, model.ttl = 30000 ms
> run
Walk to root
Walk to fetcher
Executed fetcher with skill graph.api.fetcher in 12.085 ms
Walk to fetcher-2
Executed fetcher-2 with skill graph.api.fetcher in 14.326 ms
Walk to end
{
  "output": {
    "body": {
      "address": "100 World Blvd",
      "name": "Peter",
      "accounts": [
        {
          "balance": 25032.13,
          "id": "a101",
          "type": "Saving"
        },
        {
          "balance": 6020.68,
          "id": "b202",
          "type": "Current"
        },
        {
          "balance": 120000.0,
          "id": "c303",
          "type": "C/D"
        },
        {
          "balance": 6000.0,
          "id": "d400",
          "type": "apple"
        },
        {
          "balance": 8200.0,
          "id": "e500",
          "type": "google"
        }
      ]
    }
  }
}
Graph traversal completed in 28 ms
\`\`\`

Parallelism
-----------
With the "for_each" method, the system performs the API fetches in parallel. The
default concurrency is 3; set "concurrency" in "fetcher-2" (1-30) to try other
values.

With a concurrency of 3 and five accounts, the system makes a batch of 3 followed by
a batch of 2 API requests. When you change the concurrency setting, the batch size
adjusts accordingly.

Aggregation order is guaranteed: batches execute in source-list order and responses
join in request order, so the aggregated result array preserves the order of the
source account numbers — regardless of the concurrency setting. You can see this in
the dry-run above: the account details appear in the same order as the account
numbers (a101 to e500).

Create an island to hold the data dictionary
--------------------------------------------
Wire the data dictionary and provider nodes into the graph's knowledge layer with an
island node. This is the required convention — leave no node unconnected: the island
subgraph is the graph's entity-relationship diagram, turning the graph into living
documentation of enterprise knowledge.

\`\`\`
create node dictionary
with type Island
with properties
skill=graph.island
\`\`\`

Then connect the data dictionary nodes and provider nodes to it.

\`\`\`
> connect root to dictionary with contains
node root connected to dictionary
> connect dictionary to person-name with data
node dictionary connected to person-name
> connect dictionary to person-address with data
node dictionary connected to person-address
> connect dictionary to person-accounts with data
node dictionary connected to person-accounts
> connect person-name to mdm-profile with provider
node person-name connected to mdm-profile
> connect person-address to mdm-profile with provider
node person-address connected to mdm-profile
> connect person-accounts to mdm-profile with provider
node person-accounts connected to mdm-profile
> connect dictionary to account-details with data
node dictionary connected to account-details
> connect account-details to account-details-provider with provider
node account-details connected to account-details-provider
> list connections
root -[contains]-> dictionary
root -[fetch]-> fetcher
account-details -[provider]-> account-details-provider
dictionary -[data]-> account-details
dictionary -[data]-> person-accounts
dictionary -[data]-> person-address
dictionary -[data]-> person-name
fetcher -[details]-> fetcher-2
person-accounts -[provider]-> mdm-profile
person-address -[provider]-> mdm-profile
person-name -[provider]-> mdm-profile
fetcher-2 -[complete]-> end
\`\`\`

Export the graph model
----------------------
Save the graph model by exporting it.

\`\`\`
> export graph as tutorial-6
Graph exported to /tmp/graph/tutorial-6.json
Described in /api/graph/model/tutorial-6/775-18
\`\`\`

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-6.json" to your application's
resources/graph folder. You can then test the deployed model with a curl command.

\`\`\`
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-6 \\
  -H "Content-Type: application/json" \\
  -d '{
    "person_id": 100
  }'
\`\`\`

Summary
-------
In this tutorial, you created a graph model that performs two steps of API fetching.
The first step gets the name, address and list of account numbers. The second step
uses the "for_each" method to fetch the account details for each account number, and
aggregates the results into a single array in source-list order.
`,Ft=`Tutorial 7
----------
In this tutorial, you will explore data mapping in more detail.

Exercise
--------
You will create a new graph model to test various data mapping methods.

To clear the previous graph session, click the Tools button in the top-right corner
and click the "Stop" and "Start" toggle button. A new graph session will start.

Create a root node and an end node
----------------------------------
Enter the following to create a root node and an end node.

\`\`\`
create node root
with type Root
with properties
name=tutorial-7
purpose=Demonstrate various data mapping methods
\`\`\`

\`\`\`
create node end
with type End
with properties
\`\`\`

Create a data mapper node
-------------------------
Let's try some data mapping methods. Please enter the following:

\`\`\`
create node data-mapper
with type Mapper
with properties
mapping[]=text(world) -> output.body.hello
mapping[]=input.body.profile.name -> output.body.name
mapping[]=model.none -> model.address
mapping[]=input.body.profile.address1 -> model.address[]
mapping[]=input.body.profile.address2 -> model.address[]
mapping[]=model.address -> output.body.address
mapping[]=f:now(text(local)) -> output.body.time
\`\`\`

\`mapping[]\` builds the node's data mapping statement list in "append mode": the
statements are evaluated in the order provided.

Each data mapping statement has a left-hand side (the source) and a right-hand side
(the target), separated by the "map to" indicator (\`->\`). The value of the source is
mapped to the target key.

MiniGraph uses the same data mapping syntax as Event Script. For a quick reference,
enter "help graph-data-mapper"; the full syntax is in the Event Script syntax guide:
https://accenture.github.io/mercury-composable/guides/event-script/syntax/

*Constant* — \`text(world)\` means a constant of "world". \`output.body.\` is the
namespace for the output payload when a graph finishes execution. In this example,
output.body is populated with "hello=world".

*Input* — \`input.body\` is the namespace for the input payload provided to a graph
instance when it starts.

Assuming the input payload looks like this:

\`\`\`json
{
  "profile": {
    "name": "Peter",
    "address1": "100 World Blvd",
    "address2": "New York"
  }
}
\`\`\`

The value "Peter" is mapped to the "name" field, and address1 and address2 become the
first and second elements of an array in "model.address". The \`model.\` namespace is a
temporary state machine that lives for the duration of the graph instance — use it as
a scratch buffer for data transformation.

*Output* — the mapping statement \`model.address -> output.body.address\` maps the
two-element address array into the output payload of the graph instance.

Building an array — two techniques
----------------------------------
*Direct addressing (preferred)* — set the array element index explicitly:

\`\`\`
mapping[]=input.body.profile.address1 -> model.address[0]
mapping[]=input.body.profile.address2 -> model.address[1]
mapping[]=model.address -> output.body.address
\`\`\`

Numeric indices write each value into a known slot, so the result is deterministic
and the mapping is idempotent — executing the node again simply overwrites the same
slots. Use direct addressing whenever you know where each value belongs.

*Append + clear (for append-mode workflows)* — the array append syntax (\`[]\`) adds
one element to the end of the array on every execution. That is what you want when a
workflow accumulates an unknown number of elements — but it is not idempotent: during
testing, you may execute the same node several times, and each pass would append
duplicate entries. To make an append sequence repeatable, clear the array first by
mapping a non-existent key (conventionally \`model.none\`) to it:

\`\`\`
mapping[]=model.none -> model.address
mapping[]=input.body.profile.address1 -> model.address[]
mapping[]=input.body.profile.address2 -> model.address[]
\`\`\`

Mapping a source that does not exist removes the target key — the \`model.none\` clear
idiom. This exercise deliberately uses the append + clear form so you can observe the
idiom at work.

*Plugin functions* — the left-hand side of \`f:now(text(local)) -> output.body.time\`
uses the \`f:\` syntax to execute a "plugin" function called "now". It takes the
constant value "local" and returns a local timestamp. A number of built-in data
mapping plugins are available — see the simple-plugin catalog in the Event Script
syntax guide: https://accenture.github.io/mercury-composable/guides/event-script/syntax/

Test the data mapper
--------------------
You can test the data mapper before you complete the whole graph model.

Enter the following to instantiate the graph and open a dialog box for the mock
input data.

\`\`\`
> instantiate graph
Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms
> upload mock data
Mock data loaded into 'input.body' namespace
\`\`\`

When you enter the "upload mock data" command, an input dialog box opens. Paste the
sample input payload for the "profile" of "Peter" listed above.

To confirm that you have uploaded the mock input, enter "inspect input".

\`\`\`
> inspect input
{
  "inspect": "input",
  "outcome": {
    "body": {
      "profile": {
        "address2": "New York",
        "address1": "100 World Blvd",
        "name": "Peter"
      }
    }
  }
}
\`\`\`

You can now test the data mapper by "executing" it. Enter "execute data-mapper".

\`\`\`
> execute data-mapper
ERROR: node data-mapper does not have a skill property
\`\`\`

The system rejects the request with an error message: the data-mapper node is missing
a skill.

Enter 'edit node data-mapper', copy the printed "update node" block into the input
box, add "skill=graph.data.mapper" and submit.

\`\`\`
> edit node data-mapper
update node data-mapper
with type Mapper
with properties
mapping[]=text(world) -> output.body.hello
mapping[]=input.body.profile.name -> output.body.name
mapping[]=model.none -> model.address
mapping[]=input.body.profile.address1 -> model.address[]
mapping[]=input.body.profile.address2 -> model.address[]
mapping[]=model.address -> output.body.address
mapping[]=f:now(text(local)) -> output.body.time
skill=graph.data.mapper
\`\`\`

The system will display "node data-mapper updated".

To activate the updated node, restart the graph instance by entering
'instantiate graph' and 'upload mock data'. Submit the mock input payload again.

Then execute the data-mapper again.

\`\`\`
> execute data-mapper
node data-mapper run for 0.488 ms with exit path 'next'
\`\`\`

The data-mapper runs successfully.

Inspect the model and output
----------------------------
Inspect the model and the output key-values to see what values were mapped.

\`\`\`
> inspect model
{
  "inspect": "model",
  "outcome": {
    "address": [
      "100 World Blvd",
      "New York"
    ]
  }
}
> inspect output
{
  "inspect": "output",
  "outcome": {
    "body": {
      "address": [
        "100 World Blvd",
        "New York"
      ],
      "name": "Peter",
      "hello": "world",
      "time": "2026-04-11 19:52:22.527"
    }
  }
}
\`\`\`

Connect the nodes to complete the graph model
---------------------------------------------
Enter the two connect commands below.

\`\`\`
> connect root to data-mapper with mapping
node root connected to data-mapper
> connect data-mapper to end with complete
node data-mapper connected to end
\`\`\`

The graph model is shown in the right panel.

Export the graph model
----------------------
Save the graph model by exporting it.

\`\`\`
> export graph as tutorial-7
Graph exported to /tmp/graph/tutorial-7.json
Described in /api/graph/model/tutorial-7/152-13
\`\`\`

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-7.json" to your application's
resources/graph folder. You can then test the deployed model with a curl command.

\`\`\`
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-7 \\
  -H "Content-Type: application/json" \\
  -d '{
  "profile": {
    "name": "Peter",
    "address1": "100 World Blvd",
    "address2": "New York"
  }
}'
\`\`\`

Summary
-------
In this tutorial, you created a graph model that performs data mapping. You compared
the two array-building techniques — direct addressing (preferred) and append + clear
with the \`model.none\` idiom — transformed address1 and address2 into an array, and
applied the "f:now()" plugin function to return the current time.
`,It=`Tutorial 8
----------
In this tutorial, you will use the JSON-Path search feature to retrieve key-values from the input
payload, then reshape the result with the f:listOfMap() and f:removeKey() plugins. Reshaping a
third-party API response into your own internal data contract — "impedance matching" — is one of
the most common jobs for a data mapper, and these tools let you do it without writing code.

Exercise
--------
You will import tutorial-7 and replace some data mapping statements with JSON-Path search requests.

To clear the previous graph session, click the Tools button in the top-right corner and click the
"Stop" and "Start" toggle button. A new graph session will start.

Import tutorial-7
-----------------
Enter 'import graph from tutorial-7' first.

\`\`\`
> import graph from tutorial-7
Found deployed graph model in classpath:/graph
Please export an updated version and re-import to instantiate an instance model
Graph model imported as draft
\`\`\`

Input payload
-------------
The account holder "Peter" has 2 accounts. We will assume the following input payload data
structure. You will copy-n-paste this JSON dataset when the "upload mock data" dialog box opens
later in this exercise.

\`\`\`json
{ 
  "profile": {
    "name": "Peter",
    "account": [
      {
        "id": "100",
        "amount": 18000.30,
        "description": "Time deposit",
        "type": "C/D"
      },
      {
        "id": "200",
        "amount": 62050.80,
        "description": "Saving account",
        "type": "Saving"
      }
    ]
  }
}
\`\`\`

Edit the data mapper node
-------------------------
Let's try some data mapping methods. Please enter the following:

\`\`\`
update node data-mapper
with type Mapper
with properties
mapping[]=input.body.profile.name -> output.body.name
mapping[]=$.input.body.profile.account[*].type -> model.type
mapping[]=$.input.body.profile.account[*].id -> model.id
mapping[]=$.input.body.profile.account[*].amount -> model.amount
skill=graph.data.mapper
\`\`\`

A mapping source that starts with "$." is a JSON-Path expression evaluated over the state machine.
The three JSON-Path statements above use the [*] wildcard to extract the type, id and amount from
every element of the account list in the input payload. For a simple key, prefer the plain
dot-bracket form (like the first statement) and save JSON-Path for queries that need it.

Test the data mapper
--------------------
Enter the following to instantiate the graph and open a dialog box to enter the mock input data.

\`\`\`
> instantiate graph
Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms
> upload mock data
Mock data loaded into 'input.body' namespace
\`\`\`

The first data mapping statement maps input.body.profile.name into the "name" field of the output
body. The JSON-Path statements extract the type, id and amount key-values from the account list
and map them into the model variables type, id and amount accordingly.

When you enter the "upload mock data" command, an input dialog box will open. Please paste the
sample input payload listed above.

To confirm that you have uploaded the mock input, enter "inspect input".

\`\`\`
> inspect input
{
  "inspect": "input",
  "outcome": {
    "body": {
      "profile": {
        "name": "Peter",
        "account": [
          {
            "amount": 18000.3,
            "description": "Time deposit",
            "id": "100",
            "type": "C/D"
          },
          {
            "amount": 62050.8,
            "description": "Saving account",
            "id": "200",
            "type": "Saving"
          }
        ]
      }
    }
  }
}
\`\`\`

You can now test the data mapper by executing it. Enter "execute data-mapper".

\`\`\`
> execute data-mapper
node data-mapper run for 0.589 ms with exit path 'next'
\`\`\`

The data-mapper runs successfully.

Inspect the model and output
----------------------------
You can inspect the model and the output key-values to see what values are mapped.

\`\`\`
> inspect model
{
  "inspect": "model",
  "outcome": {
    "amount": [
      18000.3,
      62050.8
    ],
    "id": [
      "100",
      "200"
    ],
    "type": [
      "C/D",
      "Saving"
    ]
  }
}
> inspect output
{
  "inspect": "output",
  "outcome": {
    "body": {
      "name": "Peter"
    }
  }
}
\`\`\`

This confirms that the JSON-Path statements have extracted the key-values from the account list
successfully. However, three parallel lists — a "map of lists" — is usually not a good schema
design: easy for an application to parse, but harder for a human to read. Let's turn it into a
proper list of maps.

Using the listOfMap plugin
--------------------------
For proper data structure representation, use the plugin f:listOfMap() to consolidate the maps of
lists into a list of maps. Update the data mapper like this:

\`\`\`
update node data-mapper
with type Mapper
with properties
mapping[]=input.body.profile.name -> output.body.name
mapping[]=$.input.body.profile.account[*].type -> model.account.type
mapping[]=$.input.body.profile.account[*].id -> model.account.id
mapping[]=$.input.body.profile.account[*].amount -> model.account.amount
mapping[]=f:listOfMap(model.account) -> output.body.account
skill=graph.data.mapper
\`\`\`

Note the extra level of key called "account" that holds the 3 lists for type, id and amount. The
f:listOfMap() plugin then consolidates the maps of lists into a list of maps.

Instantiate the graph, upload the same mock data and execute the data-mapper again. When you enter
'inspect model' and 'inspect output', you will see:

\`\`\`
> inspect model
{
  "inspect": "model",
  "outcome": {
    "account": {
      "amount": [
        18000.3,
        62050.8
      ],
      "id": [
        "100",
        "200"
      ],
      "type": [
        "C/D",
        "Saving"
      ]
    }
  }
}
> inspect output
{
  "inspect": "output",
  "outcome": {
    "body": {
      "name": "Peter",
      "account": [
        {
          "amount": 18000.3,
          "id": "100",
          "type": "C/D"
        },
        {
          "amount": 62050.8,
          "id": "200",
          "type": "Saving"
        }
      ]
    }
  }
}
\`\`\`

This illustrates that the listOfMap plugin can perform simple data transformation. It is handy
when your graph model uses API fetchers to retrieve data from multiple sources: without writing
code, you can group data from different data structures into the shape your consumers expect.

Using the removeKey plugin
--------------------------
When the data comes from a single source, it is even easier to use the f:removeKey() plugin to
drop the unwanted keys directly. Its form is:

\`\`\`
f:removeKey(source, text(key1), text(key2), ...)
\`\`\`

It removes the named keys from a map — or from every map in a list — and returns a copy of the
data structure. Here it strips the "description" field from every account:

\`\`\`
mapping[]=f:removeKey(input.body.profile.account, text(description)) -> output.body.account
\`\`\`

Let's prove this by editing the data-mapper again. We add a new data mapping statement at the end
to map the alternative solution to the "account2" field in the output payload.

\`\`\`
update node data-mapper
with type Mapper
with properties
mapping[]=input.body.profile.name -> output.body.name
mapping[]=$.input.body.profile.account[*].type -> model.account.type
mapping[]=$.input.body.profile.account[*].id -> model.account.id
mapping[]=$.input.body.profile.account[*].amount -> model.account.amount
mapping[]=f:listOfMap(model.account) -> output.body.account
mapping[]=f:removeKey(input.body.profile.account, text(description)) -> output.body.account2
skill=graph.data.mapper
\`\`\`

Do 'instantiate graph' and 'upload mock data' with the same input payload. Then
'execute data-mapper' and 'inspect output' to see the outcome.

\`\`\`
> execute data-mapper
node data-mapper run for 2.826 ms with exit path 'next'
> inspect output
{
  "inspect": "output",
  "outcome": {
    "body": {
      "name": "Peter",
      "account2": [
        {
          "amount": 18000.3,
          "id": "100",
          "type": "C/D"
        },
        {
          "amount": 62050.8,
          "id": "200",
          "type": "Saving"
        }
      ],
      "account": [
        {
          "amount": 18000.3,
          "id": "100",
          "type": "C/D"
        },
        {
          "amount": 62050.8,
          "id": "200",
          "type": "Saving"
        }
      ]
    }
  }
}
\`\`\`

Note that "account" and "account2" have the same key-values and data structure. This confirms that
the "description" key-value has been removed from each map in the list successfully.

Export the graph model
----------------------
As a good practice, you may save the graph model by exporting it.

\`\`\`
> export graph as tutorial-8
Graph exported to /tmp/graph/tutorial-8.json
Described in /api/graph/model/tutorial-8/315-6
\`\`\`

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-8.json" to your application's
\`resources/graph\` folder. You can then test the deployed model with a curl command.

\`\`\`
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-8 \\
  -H "Content-Type: application/json" \\
  -d '{ 
  "profile": {
    "name": "Peter",
    "account": [
      {
        "id": "100",
        "amount": 18000.30,
        "description": "Time deposit",
        "type": "C/D"
      },
      {
        "id": "200",
        "amount": 62050.80,
        "description": "Saving account",
        "type": "Saving"
      }
    ]
  }
}'
\`\`\`

Summary
-------
In this tutorial, you have used JSON-Path retrieval to extract key-values from a list, applied the
f:listOfMap() plugin to consolidate maps of lists into a list of maps, and used the f:removeKey()
plugin to remove unwanted key-values from a list of maps — the building blocks for reshaping a
third-party API response into your internal data contract.

Note that JSON-Path also supports value comparison for selective key-value retrieval. Please refer
to a JSON-Path syntax reference on the web for more details.
`,Lt=`Tutorial 9
----------
In this tutorial, you will create a reusable module — a formula authored once, then borrowed by
any node that needs it.

Exercise
--------
You will create a reusable "addition" module, call it from a compute node with the EXECUTE
statement, and organize the module under an island node.

To clear the previous graph session, click the Tools button in the top-right corner and click the
"Stop" and "Start" toggle button. A new graph session will start.

What is a reusable module?
--------------------------
A module is a node with the graph.math skill that stays off the execution path. For a frequently
used math formula or boolean operation, you can save the "common logic" in one or more module
nodes and export them as a common graph model. When you design a new graph model, you can import
the modules you need from that common model.

This is a best practice for common computation and decision logic: developers do not re-invent the
same formula, and the shared modules encourage quality control and governance.

For this tutorial, we will skip exporting a common graph model and focus on creating a reusable
module and using it in a graph model.

Create a root node and an end node
----------------------------------
Enter the following to create a root node and an end node.

\`\`\`
create node root
with type Root
with properties
name=tutorial-9
purpose=Demonstrate use of modules
\`\`\`

\`\`\`
create node end
with type End
\`\`\`

Create a reusable module
-------------------------
You will create a simple "addition" module that adds two numbers and saves the result in a
variable called "sum".

\`\`\`
create node addition
with type Module
with properties
skill=graph.math
statement[]=COMPUTE: sum -> {model.a} + {model.b}
\`\`\`

Test the module
---------------
Enter the following to start the graph model and set two numbers in the variables "a" and "b" of
the state machine's "model" namespace.

\`\`\`
instantiate graph
int(10) -> model.a
int(20) -> model.b
\`\`\`

You can then test the module using 'execute addition'.

\`\`\`
> execute addition
node addition run for 0.312 ms with exit path 'next'
\`\`\`

Then you can inspect the node.

\`\`\`
> inspect addition
{
  "inspect": "addition",
  "outcome": {
    "result": {
      "sum": 30.0
    },
    "decision": "next"
  }
}
\`\`\`

The module adds the two numbers and saves the result "30.0" into the variable "sum" in the node's
result set. (When executed directly, the result lands on the module itself — the next step shows
what changes when another node executes it.)

Using the new module
--------------------
You will create a new node that uses the module.

\`\`\`
create node compute
with type Compute
with properties
skill=graph.math
statement[]=MAPPING: input.body.a -> model.a
statement[]=MAPPING: input.body.b -> model.b
statement[]=EXECUTE: addition
statement[]=MAPPING: compute.result.sum -> output.body.sum
\`\`\`

This node maps the input parameters "a" and "b" into the model variables "a" and "b", executes the
module "addition", then maps the computed value to the output payload "output.body.sum". Note the
last statement reads compute.result.sum — not addition.result.sum — for the reason shown next.

Test the compute node
---------------------
You will instantiate the graph model like this:

\`\`\`
instantiate graph
int(10) -> input.body.a
int(20) -> input.body.b
\`\`\`

Then enter 'execute compute'. It maps the input parameters to the model variables and executes the
module "addition" that adds the two model variables together.

Inspect the result
------------------
The result is saved to the variable "sum" under the "compute" node instead of the module
"addition". EXECUTE runs the module's statements in the caller's context: any COMPUTE result lands
on the invoking node (compute.result.sum here), and the module's own namespace stays empty — the
compute node just borrows the logic from the module.

\`\`\`
> inspect compute
{
  "inspect": "compute",
  "outcome": {
    "result": {
      "sum": 30.0
    },
    "decision": "next"
  }
}
> inspect model
{
  "inspect": "model",
  "outcome": {
    "a": 10,
    "b": 20
  }
}
> inspect addition
{
  "inspect": "addition",
  "outcome": {}
}
> inspect output
{
  "inspect": "output",
  "outcome": {
    "body": {
      "sum": 30.0
    }
  }
}
\`\`\`

The module works as expected.

Connect the nodes
-----------------
You will connect the nodes with the following commands:

\`\`\`
connect root to compute with calculate
connect compute to end with finish
\`\`\`

Test the completed model
------------------------
You will enter the following to test the whole model ('start' is an alias of 'instantiate').

\`\`\`
start graph
int(10) -> input.body.a
int(20) -> input.body.b
\`\`\`

Then enter 'run' to do a 'dry-run' from the root to the end node.

\`\`\`
> run
Walk to root
Walk to compute
Executed compute with skill graph.math in 0.387 ms
Walk to end
{
  "output": {
    "body": {
      "sum": 30.0
    }
  }
}
Graph traversal completed in 7 ms
\`\`\`

Check the nodes and connections
-------------------------------
Enter the following to show the nodes and connections.

\`\`\`
> list nodes
root [Root]
addition [Module]
compute [Compute]
end [End]
> list connections
root -[calculate]-> compute
compute -[finish]-> end
\`\`\`

Note that the module "addition" is not part of the traversal path — the compute node that executes
it is. However, the convention is to leave no node unconnected: 'export' fails if any node is an
orphan, and off-path nodes belong in the graph's knowledge structure so the model documents
itself. The next step wires the module in.

Create an island to hold modules
--------------------------------
You will create an island node to organize one or more module nodes. An island is isolated from
graph traversal, so the execution path is unaffected.

\`\`\`
create node modules
with type Island
with properties
skill=graph.island
\`\`\`

Then connect the root to the island, and the island to the module.

\`\`\`
> connect root to modules with contains
node root connected to modules
> connect modules to addition with contains
node modules connected to addition
> list connections
root -[calculate]-> compute
root -[contains]-> modules
modules -[contains]-> addition
compute -[finish]-> end
\`\`\`

Export the graph model
----------------------
As a good practice, you may save the graph model by exporting it.

\`\`\`
> export graph as tutorial-9
Graph exported to /tmp/graph/tutorial-9.json
Described in /api/graph/model/tutorial-9/359-15
\`\`\`

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-9.json" to your application's
\`resources/graph\` folder. You can then test the deployed model with a curl command.

\`\`\`
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-9 \\
  -H "Content-Type: application/json" \\
  -d '{ 
    "a": 10,
    "b": 20
}'
\`\`\`

Summary
-------
In this tutorial, you have created a graph model with a compute node that executes a reusable
module. You have seen that EXECUTE runs the module's statements in the caller's context — the
result lands on the invoking node — and you have organized the module under an island so that no
node is left unconnected.
`,Rt=`Update a node
-------------
Replace the definition of an existing node. This multi-line command has the
same shape as 'create node' (see 'help create'): enter all lines as one
block, and the node takes the type and properties you provide.

Syntax
------
\`\`\`
update node {name}
with type {type}
with properties
{key1}={value1}
{key2}={value2}
\`\`\`

Example
-------
\`\`\`
update node greeting
with type Task
with properties
skill=graph.task
task=no.op
input[]=input.body -> *
output[]=result -> output.body
\`\`\`

Notes
-----
- Node names are matched case-insensitively; use lowercase letters, digits
  and hyphen ('root' and 'end' are reserved for the root and end nodes).
- Types are descriptive labels, conventionally Capitalized; the type and
  properties are validated by the node's skill, if any.
- A node has zero or one skill, set with skill={route}.
- 'with properties' and the key lines are optional; a key[]=entry line
  appends one entry to the list "key"; wrap a multi-line value in triple
  single quotes ('''). Values may use the Event Script constant syntax.
- Tip: 'edit node {name}' prints an existing node as a ready-to-edit
  'update node' command (see 'help edit').
`,zt=`Upload mock data
----------------
Print the URLs for uploading mock input to the current graph instance - a
JSON payload as 'input.body', and optional request headers as 'input.header'
- convenient when the mock input is too large to seed line by line, or when
the graph reads a header.

Syntax
------
\`\`\`
upload mock data
\`\`\`

Example
-------
\`\`\`
> upload mock data
You may upload JSON payload -> POST /api/mock/{name} (mock headers, a JSON object of text values -> POST /api/mock/{name}?namespace=header)
\`\`\`

Notes
-----
- Requires a graph instance (see 'help instantiate').
- An HTTP POST of a JSON payload (a map or a list) to the first URL replaces
  the instance's 'input.body'; the console confirms with "Mock data loaded
  into 'input.body' namespace".
- An HTTP POST of a JSON object of text values to the same URL with
  '?namespace=header' replaces the instance's 'input.header'; the console
  confirms with "Mock data loaded into 'input.header' namespace". Header
  names are kept as given and read case-insensitively by the graph, exactly
  as a real request's headers are. Any other namespace, or a header payload
  that is not an object of text values, is refused (HTTP 400).
- To seed model variables, or small inputs line by line, use the
  'instantiate graph' command (see 'help instantiate'): its
  '{value} -> input.body.{key}' and '{value} -> input.header.{name}' lines
  do the same by hand.
- In a collaborative session (see 'help session') an uploaded payload loads
  into every member's graph instance - the primary and all its subscribers -
  and each member's console confirms it. In the Playground UI the toolbar's
  Upload button opens the upload form for your own session only, with the
  JSON body and optional header rows; uploading is optional (a graph that
  reads no input runs without it).
`,Bt=`MiniGraph
---------
A mini-graph is a property graph designed to run entirely in memory
(default capacity: 750 nodes).

A graph model describes a business use case using graph methodology.
Optionally, you may give a node a special skill so it reacts to incoming
events. A skill is a property with the label "skill" whose value is a
composable function route name.

An instance model is an instance of a graph model used to process one
specific business use case or transaction. In the Playground you create it
with the "instantiate" command, optionally seeding mock input; in a deployed
application it is created when an incoming event arrives. Input data
attributes map to properties of one or more nodes.

Execution of an instance model starts from the root node and walks the graph
until it reaches the end node. The result of the end node is returned to the
calling party.

For a model to be meaningful, at least one node should have a skill to
process the data attributes of other nodes (the "data entities").

For more information about each feature, try the following help topics.

Keyboard shortcuts
------------------
- \`Ctrl + M\` - Toggle the minimap while viewing the Graph tab
- \`Ctrl + backtick\` - Toggle the Help panel

For graph model
---------------
- help create (node)
- help update (node)
- help edit (node)
- help delete (node, connection or cache)
- help connect (node-A to node-B)
- help list (nodes, connections, graphs, flows)
- help export (graph model as JSON for deployment)
- help import (graph or node)
- help package (graph sets: pack, inspect and deploy)
- help describe (graph, node, connection or skill)
- help data-dictionary
- help session (display, subscribe or reset session)

For instance model
------------------
- help instantiate (create an instance from the current graph model)
- help upload (mock data)
- help execute (the skill of one node in isolation, for functional testing)
- help inspect (state machine: node properties, input, output and model namespaces)
- help run (traverse a graph instance from the root node to the end node)
- help seen (display the nodes that have been seen or executed)

Built-in skills
---------------
1. graph.data.mapper - map data from one node or namespace to another
2. graph.math - compute and branch with a fast built-in math/boolean expression engine
3. graph.js - deprecated (not registered in the Rust engine); use graph.math or graph.task instead
4. graph.api.fetcher - make API calls to other systems via Dictionary and Provider nodes
5. graph.extension - delegate to another graph model or an Event Script flow
6. graph.island - marks the knowledge layer; the node leads to isolated nodes and traversal pauses there
7. graph.join - wait for completion of all nodes that connect to it (parallel-branch barrier)
8. graph.task - invoke a composable function through its route name
9. graph.suspend - persist workflow state at a suspension point (the reserved 'suspend' node)
10. graph.resume - restore workflow state and continue past the suspension point

For skill details, use the hyphenated help topics, e.g. 'help graph-math',
'help graph-api-fetcher', or 'describe skill {route}'.

Tutorials
---------
- help tutorial 1 (your first 'hello world' graph model)
- help tutorial 2 (deploying a graph model)
- help tutorial 3 (data dictionary, provider and API fetcher)
- help tutorial 4 (decision-making with math and boolean expressions)
- help tutorial 5 (parallel processing with a join barrier)
- help tutorial 6 (iterative API fetching with the 'for_each' keyword)
- help tutorial 7 (data mapping)
- help tutorial 8 (JSON-Path key-value retrieval and search)
- help tutorial 9 (reusable 'modules')
- help tutorial 10 (graph extension)
- help tutorial 11 (flow extension)
- help tutorial 12 (custom error handling)
- help tutorial 13 (invoking a composable function with the graph.task skill)
- help tutorial 14 (workflow suspension - a purchase workflow with three human checkpoints)
`,Vt=[`# JSON-Path Playground Overview`,``,`Use the JSON-Path Playground to load a JSON or XML document and evaluate expressions against it.`,`This starter Overview is intentionally the only section for now and can grow into a fuller guide later.`,``,`## Quick start`,``,`1. Click **Start** to connect to the JSON-Path playground.`,`2. Paste a document into the Payload Editor, or choose a Quick load sample.`,"3. Enter `load` to send the document to the active session.","4. Enter a JSONPath expression beginning with `$`, such as `$.response.user.name`.",``,`## Example expressions`,``,`| Expression | Purpose |`,`| --- | --- |`,"| `$.response` | Read the loaded response object |","| `$.response.items[*]` | Select every item in an array |","| `$.response.items[0].name` | Read a field from the first item |",``,`## Help controls`,``,"- `Ctrl + backtick` - Toggle the Help panel",`- Use the maximize button to expand Help and the close button to return to the editor`].join(`
`),Ht=Object.assign({"../../../src/main/resources/help/help connect.md":Qe,"../../../src/main/resources/help/help create.md":$e,"../../../src/main/resources/help/help data-dictionary.md":et,"../../../src/main/resources/help/help delete.md":tt,"../../../src/main/resources/help/help describe.md":nt,"../../../src/main/resources/help/help edit.md":rt,"../../../src/main/resources/help/help execute.md":it,"../../../src/main/resources/help/help export.md":at,"../../../src/main/resources/help/help graph-api-fetcher.md":ot,"../../../src/main/resources/help/help graph-data-mapper.md":st,"../../../src/main/resources/help/help graph-extension.md":ct,"../../../src/main/resources/help/help graph-island.md":lt,"../../../src/main/resources/help/help graph-join.md":ut,"../../../src/main/resources/help/help graph-js.md":dt,"../../../src/main/resources/help/help graph-math.md":ft,"../../../src/main/resources/help/help graph-resume.md":pt,"../../../src/main/resources/help/help graph-suspend.md":mt,"../../../src/main/resources/help/help graph-task.md":ht,"../../../src/main/resources/help/help import.md":gt,"../../../src/main/resources/help/help inspect.md":_t,"../../../src/main/resources/help/help instantiate.md":vt,"../../../src/main/resources/help/help list.md":yt,"../../../src/main/resources/help/help package.md":bt,"../../../src/main/resources/help/help run.md":xt,"../../../src/main/resources/help/help seen.md":St,"../../../src/main/resources/help/help session.md":Ct,"../../../src/main/resources/help/help tutorial 1.md":wt,"../../../src/main/resources/help/help tutorial 10.md":Tt,"../../../src/main/resources/help/help tutorial 11.md":Et,"../../../src/main/resources/help/help tutorial 12.md":Dt,"../../../src/main/resources/help/help tutorial 13.md":Ot,"../../../src/main/resources/help/help tutorial 14.md":kt,"../../../src/main/resources/help/help tutorial 2.md":At,"../../../src/main/resources/help/help tutorial 3.md":jt,"../../../src/main/resources/help/help tutorial 4.md":Mt,"../../../src/main/resources/help/help tutorial 5.md":Nt,"../../../src/main/resources/help/help tutorial 6.md":Pt,"../../../src/main/resources/help/help tutorial 7.md":Ft,"../../../src/main/resources/help/help tutorial 8.md":It,"../../../src/main/resources/help/help tutorial 9.md":Lt,"../../../src/main/resources/help/help update.md":Rt,"../../../src/main/resources/help/help upload.md":zt,"../../../src/main/resources/help/help.md":Bt});function Ut(e){let t=e.split(`/`);return(t[t.length-1]??e).replace(/\.md$/,``)}var Wt=Object.fromEntries(Object.entries(Ht).map(([e,t])=>[Ut(e),t])),Gt={help:Vt};function Kt(e){return e===`json-path`?Gt:Wt}function qt(e,t=`minigraph`){let n=e===``?`help`:`help ${e}`;return Kt(t)[n]??null}var Jt=Object.keys(Wt).filter(e=>e!==`help`).map(e=>e.replace(/^help\s+/,``)).sort(),Yt=[{id:`overview`,label:`Overview`},{id:`graph-model`,label:`Graph Model`},{id:`graph-skills`,label:`Graph Skills`},{id:`instance-model`,label:`Instance Model`},{id:`tutorials`,label:`Tutorials`,chipStripLabel:`Chapters`}],Xt=[{id:`overview`,label:`Overview`}];function Zt(e=`minigraph`){return e===`json-path`?Xt:Yt}var Qt=new Set([`execute`,`inspect`,`instantiate`,`run`,`seen`,`upload`]);function $t(e,t=`minigraph`){return t===`json-path`||e===``?`overview`:e.startsWith(`tutorial `)?`tutorials`:e.startsWith(`graph-`)?`graph-skills`:Qt.has(e)?`instance-model`:`graph-model`}function en(e,t=`minigraph`){if(e===`overview`)return[``];if(t===`json-path`)return[];let n=Jt.filter(n=>$t(n,t)===e);return e===`tutorials`?[...n].sort((e,t)=>parseInt(e.replace(/^tutorial\s+/,``),10)-parseInt(t.replace(/^tutorial\s+/,``),10)):n}function tn(e,t){return e===``?`Overview`:t===`tutorials`?e.replace(/^tutorial\s+/,``):e}function nn(e=`minigraph`){return Zt(e).flatMap(t=>en(t.id,e))}nn();function rn(e,t,n=`minigraph`){if(!t)return null;let r=Ze(e);return r===null||qt(r,n)===null?null:r}function an({bus:e,setHelpTopic:t,onTabSwitch:n,enabled:r=!0,contentProfile:i=`minigraph`}){let a=(0,P.useRef)(n);(0,P.useEffect)(()=>{a.current=n}),(0,P.useEffect)(()=>{if(r)return e.on(`command.helpOrDescribe`,e=>{let n=rn(e.commandText,!0,i);n!==null&&(t(n),a.current())})},[e,t,r,i])}function on({ctx:e,navigate:t,addToast:n,wsPath:r}){let i=ue.find(e=>e.tabs.includes(`payload`)&&e.supportsUpload),a=(0,P.useRef)(null),o=i?.wsPath;(0,P.useEffect)(()=>{if(!(!o||!a.current)&&e.getSlot(o).phase===`connected`){let{wsPath:r,json:o}=a.current;a.current=null,e.setPendingPayload(r,o),t(i.path),n(`JSON loaded into JSON-Path editor ✓`,`success`)}},[o,e,t,n,i]);let s=(0,P.useCallback)(r=>{if(!i)return;let o=e.getSlot(i.wsPath);o.phase===`connected`?(e.setPendingPayload(i.wsPath,r),t(i.path),n(`JSON loaded into JSON-Path editor ✓`,`success`)):o.phase===`connecting`?(a.current={wsPath:i.wsPath,json:r},n(`Updated pending JSON transfer — latest payload will open when connected`,`info`)):(a.current={wsPath:i.wsPath,json:r},e.connect(i.wsPath,n),n(`Connecting to JSON-Path Playground…`,`info`))},[e,t,n,i]);return{handleSendToJsonPath:i&&r!==i.wsPath?s:void 0}}function sn({addToast:e}){let[t,n]=(0,P.useState)(null),r=(0,P.useRef)(null),i=(0,P.useRef)([]),a=(0,P.useRef)(null),[o,s]=(0,P.useState)(new Set),c=(0,P.useCallback)(e=>{let t=r.current;if(t!==null){t!==e&&!i.current.includes(e)&&i.current.push(e);return}a.current=document.activeElement,r.current=e,n(e)},[]),l=(0,P.useCallback)(()=>{let e=i.current.shift()??null;r.current=e,n(e),e===null&&setTimeout(()=>a.current?.focus(),0)},[]);return{uploadPanelPath:t,successfulUploadPaths:o,handleOpenUploadPanel:c,handleCloseUploadPanel:(0,P.useCallback)(()=>{l()},[l]),handleCloseUploadPath:(0,P.useCallback)(e=>{if(r.current===e)return l(),!0;let t=i.current.indexOf(e);return t===-1?!1:(i.current.splice(t,1),!0)},[l]),handleUploadSuccess:(0,P.useCallback)(t=>{let n=r.current;n&&s(e=>new Set([...e,n])),l(),e(`Mock data uploaded successfully ✓`,`success`)},[e,l]),handleUploadError:(0,P.useCallback)(t=>{e(`Upload failed: ${t}`,`error`)},[e]),resetSuccessfulPaths:(0,P.useCallback)(()=>{s(new Set)},[])}}function cn({bus:e,connected:t,appendMessage:n,addToast:r}){let i=(0,P.useRef)(null),a=(0,P.useRef)(!1),o=(0,P.useRef)(n);(0,P.useEffect)(()=>{o.current=n},[n]);let s=(0,P.useRef)(r);(0,P.useEffect)(()=>{s.current=r},[r]),(0,P.useEffect)(()=>{t||(i.current?.abort(),i.current=null,a.current=!1)},[t]),(0,P.useEffect)(()=>()=>{i.current?.abort()},[]),(0,P.useEffect)(()=>e.on(`payload.large`,e=>{if(a.current)return;let{apiPath:t,byteSize:n}=e;i.current?.abort();let r=new AbortController;i.current=r;let c=(n/(1024*1024)).toFixed(2);s.current(`Fetching large payload (${c} MB)…`,`info`),a.current=!0,fetch(t,{signal:r.signal}).then(e=>{if(!e.ok)throw Error(`HTTP ${e.status}`);return e.text()}).then(e=>{if(!e.trim())throw Error(`empty response body`);let t=e;try{t=JSON.stringify(JSON.parse(e),null,2)}catch{}o.current(t),a.current=!1,i.current=null}).catch(e=>{e.name!==`AbortError`&&(a.current=!1,i.current=null,o.current(`ERROR: payload fetch failed — ${e.message}`),s.current(`Payload fetch failed: ${e.message}`,`error`))})}),[e])}function ln(e){let[t,n]=ce(e,{}),r=(0,P.useCallback)(e=>{n(t=>({...t,[e]:{name:e,savedAt:new Date().toISOString()}}))},[n]),i=(0,P.useCallback)(e=>{n(t=>{let n={...t};return delete n[e],n})},[n]),a=(0,P.useCallback)(e=>Object.prototype.hasOwnProperty.call(t,e),[t]);return{savedGraphs:(0,P.useMemo)(()=>Object.values(t).sort((e,t)=>new Date(t.savedAt).getTime()-new Date(e.savedAt).getTime()),[t]),saveGraph:r,deleteGraph:i,hasGraph:a}}var un={importedName:null,lastSavedName:null,isSaved:!1,untitledSlotConsumed:!1};function dn(e,t){switch(t.type){case`imported`:return{...e,importedName:t.name,lastSavedName:null,isSaved:!1};case`exported`:return{...e,lastSavedName:t.name,isSaved:!0,untitledSlotConsumed:e.untitledSlotConsumed||t.consumesUntitled};case`dirty`:return{...e,isSaved:!1};case`reset`:return{...un}}}var fn=new Map;function pn(e,t,n){return t&&n!==null&&e===n}function mn(e,t){let n=e.on(`command.importGraph`,e=>{t.onImported(e.graphName)}),r=e.on(`graph.exported`,e=>{t.onExported(e.graphName)}),i=e.on(`graph.mutation`,()=>{t.onDirty()}),a=e.on(`session.reset`,()=>{t.onReset()});return()=>{n(),r(),i(),a()}}function hn(e,t,n,r){let[i,a]=ce(e,1),[o,s]=(0,P.useState)(()=>{let t=fn.get(e);return t&&pn(t.connectionEpoch,n,r)?t.state:{...un}}),c=(0,P.useRef)(o),l=(0,P.useCallback)(t=>{let n=dn(c.current,t);c.current=n,t.type===`reset`?fn.delete(e):r!==null&&fn.set(e,{connectionEpoch:r,state:n}),s(n)},[r,e]),u=(0,P.useCallback)(e=>{l({type:`exported`,name:e,consumesUntitled:e===`untitled-${i}`})},[i,l]),d=(0,P.useCallback)(()=>{let t=fn.get(e)?.state;(c.current.untitledSlotConsumed||t?.untitledSlotConsumed)&&a(e=>e+1),l({type:`reset`})},[a,e,l]),f=(0,P.useRef)(r);return(0,P.useEffect)(()=>{let t=fn.get(e)?.connectionEpoch;(!n||f.current!==r||t!==void 0&&t!==r)&&d(),f.current=r},[n,r,d,e]),(0,P.useEffect)(()=>mn(t,{onImported:e=>l({type:`imported`,name:e}),onExported:u,onDirty:()=>l({type:`dirty`}),onReset:d}),[t,d,u,l]),{defaultName:o.lastSavedName??o.importedName??`untitled-${i}`,savedName:o.isSaved?o.lastSavedName:null,resetName:d}}var gn=new Set([`description`,`question`,`purpose`]),_n=`input.body`,vn=`input.header`;function yn(e){return/[A-Za-z0-9_.]/.test(e)}function bn(e){return/[A-Za-z0-9_.*\[\]-]/.test(e)}function xn(e,t,n){let r=0;for(;r<e.length;){let i=e.indexOf(t,r);if(i===-1)return;let a=i>0?e[i-1]:``,o=i>1?e[i-2]:``,s=e[i+t.length]??``;if(a&&yn(a)&&!(a===`.`&&o===`$`)||s&&/[A-Za-z0-9_]/.test(s)){r=i+t.length;continue}let c=i+t.length;for(;c<e.length&&bn(e[c]);)c+=1;let l=e.slice(i,c);for(;l.endsWith(`.`)||l.endsWith(`-`)||l.endsWith(`[`);)l=l.slice(0,-1);for(;l.endsWith(`]`)&&!l.includes(`[`);)l=l.slice(0,-1);n.add(l),r=Math.max(c,i+t.length)}}function Sn(e,t,n,r){if(!(r&&gn.has(r.toLowerCase()))){if(typeof e==`string`){xn(e,t,n);return}if(Array.isArray(e)){for(let r of e)Sn(r,t,n);return}if(typeof e==`object`&&e)for(let[r,i]of Object.entries(e))Sn(i,t,n,r)}}function Cn(e,t){if(!e)return[];let n=new Set;for(let r of e.nodes)Sn(r.properties,t,n);return Array.from(n).sort()}function wn(e){return Cn(e,_n)}function Tn(e){return Cn(e,vn).map(e=>e===vn?`*`:e.slice(13))}var En={instantiate:`instantiate graph`,run:`run`},Dn=/^Graph instance created\. Loaded (\d+) mock (?:entry|entries), model\.ttl = (\d+) ms$/,On=/^Graph traversal completed in (\d+) ms$/,kn=/^ERROR:\s*(.+)$/;function An(e){let t=e.match(Dn);return t?{mockEntries:Number.parseInt(t[1],10),ttlMs:Number.parseInt(t[2],10)}:null}function jn(e){return e===`Graph instance cleared`}function Mn(e){let t=e.match(On);return t?{status:`completed`,elapsedMs:Number.parseInt(t[1],10)}:e.startsWith(`Graph traversal aborted`)?{status:`aborted`,elapsedMs:null}:null}function Nn(e){return e.match(kn)?.[1]?.trim()||null}function Pn(e){let t=e.trim().toLowerCase();return t===En.instantiate||t===`${En.instantiate}...`}function Fn(e){return e.trim().toLowerCase()===En.run}var In=1e4,Ln={phase:`idle`,pendingSignal:null,ownAction:!1,invalidated:!1};function Rn({enabled:e,bus:t,connected:n,connectionEpoch:r,graphData:i,graphIdentity:a,isPrimary:o,sendRawText:s,addToast:c}){let[l,u]=(0,P.useState)(Ln),d=(0,P.useRef)(Ln),f=(0,P.useRef)(s),p=(0,P.useRef)(c),m=(0,P.useMemo)(()=>wn(i),[i]),h=(0,P.useMemo)(()=>Tn(i),[i]),g=m.length>0||h.length>0,_=(0,P.useRef)(m),v=(0,P.useRef)(g);(0,P.useEffect)(()=>{f.current=s},[s]),(0,P.useEffect)(()=>{p.current=c},[c]),(0,P.useEffect)(()=>{_.current=m},[m]),(0,P.useEffect)(()=>{v.current=g},[g]);let y=(0,P.useCallback)(e=>{d.current=e,u(e)},[]),b=(0,P.useCallback)(()=>{y({...Ln})},[y]),x=(0,P.useCallback)(()=>{let e=d.current;if(e.pendingSignal!==null){y({...e,phase:`outcome-uncertain`,invalidated:!0});return}b()},[b,y]),S=(0,P.useCallback)((e,t,n)=>(y(t),f.current(e)?!0:(b(),p.current(n,`error`),!1)),[b,y]),C=e&&n&&i!==null,w=C&&(l.phase===`idle`||l.phase===`ready`),ee=C&&l.phase===`ready`,T=C&&l.phase===`ready`,E=e?i?n?``:`Connect first to run the graph`:`Load a graph first`:`Graph run controls are unavailable`,D=(0,P.useCallback)(()=>!T||d.current.phase!==`ready`?!1:S(En.run,{phase:`running`,pendingSignal:`run-terminal`,ownAction:!0,invalidated:!1},`Could not run graph because the WebSocket is not open.`),[T,S]),O=(0,P.useCallback)(()=>{if(!w)return!1;let e=d.current;return e.phase!==`idle`&&e.phase!==`ready`?!1:S(En.instantiate,{phase:`instantiating`,pendingSignal:`instance-created`,ownAction:!0,invalidated:!1},`Could not instantiate graph because the WebSocket is not open.`)},[w,S]);(0,P.useEffect)(()=>{let e=t.on(`graph.instance.created`,()=>{let e=d.current;if(e.pendingSignal===`instance-created`){if(e.invalidated){b();return}y({phase:`ready`,pendingSignal:null,ownAction:!1,invalidated:!1}),e.ownAction&&p.current(v.current?`Graph instantiated. Upload mock input if the run needs it, then run.`:`Graph instantiated and ready to run.`,`success`)}}),n=t.on(`graph.instance.cleared`,b),r=t.on(`graph.mutation`,x),i=t.on(`session.reset`,b),a=t.on(`graph.exported`,x),o=t.on(`graph.run.terminal`,e=>{let t=d.current;if(t.pendingSignal!==`run-terminal`)return;let n=!t.invalidated&&e.status===`aborted`;b(),n&&p.current(`Graph run aborted. See the console for details.`,`error`)}),s=t.on(`command.error`,e=>{let t=d.current;if(t.pendingSignal!==`instance-created`)return;let n=!t.invalidated;b(),n&&p.current(`Could not instantiate graph: ${e.message}`,`error`)}),c=t.on(`command.echo`,e=>{let t=d.current;t.phase!==`idle`&&t.phase!==`ready`||(Pn(e.commandText)?y({phase:`instantiating`,pendingSignal:`instance-created`,ownAction:!1,invalidated:!1}):Fn(e.commandText)&&y({phase:`running`,pendingSignal:`run-terminal`,ownAction:!1,invalidated:!1}))});return()=>{e(),n(),r(),i(),a(),o(),s(),c()}},[t,x,b,y]),(0,P.useEffect)(()=>{if(l.phase!==`instantiating`)return;let e=setTimeout(()=>{let e=d.current;e.phase===`instantiating`&&(y({...e,phase:`outcome-uncertain`}),p.current(`Graph setup is taking longer than expected. Waiting for the backend outcome…`,`info`))},In);return()=>clearTimeout(e)},[l.phase,y]);let k=(0,P.useRef)(a);(0,P.useEffect)(()=>{k.current!==a&&x(),k.current=a},[a,x]);let A=(0,P.useRef)(r);(0,P.useEffect)(()=>{A.current!==r&&b(),A.current=r},[r,b]);let te=(0,P.useRef)(o);(0,P.useEffect)(()=>{te.current!==o&&b(),te.current=o},[o,b]),(0,P.useEffect)(()=>{!e||!n?b():i||x()},[e,n,i,b,x]);let ne=l.phase!==`idle`&&l.phase!==`ready`;return{phase:l.phase,ready:l.phase===`ready`,busy:ne,canInteract:C,canInstantiate:w,canUpload:ee,canRun:T,disabledReason:E,inputBodyPaths:m,inputHeaderNames:h,runGraph:D,instantiateGraph:O}}function zn(e,t){return e.on(`graph.exported`,e=>t(e.graphName))}function Bn({bus:e,connected:t,sendRawText:n,saveGraph:r,addToast:i}){let a=(0,P.useRef)(null),o=(0,P.useCallback)(e=>{if(!t){i(`Save failed: connection required to export graph`,`error`);return}let r=setTimeout(()=>{a.current!==null&&(a.current=null,i(`Save failed: export confirmation timed out`,`error`))},1e4);a.current={graphName:e,timeoutId:r},n(`export graph as ${e}`)},[t,n,i]);return(0,P.useEffect)(()=>{if(r!==null)return zn(e,r)},[e,r]),(0,P.useEffect)(()=>e.on(`graph.exported`,e=>{if(a.current===null||e.graphName!==a.current.graphName)return;clearTimeout(a.current.timeoutId);let t=a.current.graphName;a.current=null,i(`Graph saved as "${t}"`,`success`)}),[e,i]),(0,P.useEffect)(()=>e.on(`graph.export.failed`,e=>{a.current!==null&&(clearTimeout(a.current.timeoutId),a.current=null,e.reason===`invalid-name`?i(`Save failed: invalid filename (a–z, A–Z, 0–9, hyphen only)`,`error`):i(`Save failed: root node name does not match existing graph`,`error`))}),[e,i]),(0,P.useEffect)(()=>{!t&&a.current!==null&&(clearTimeout(a.current.timeoutId),a.current=null,i(`Save failed: connection closed before export confirmation`,`error`))},[t,i]),(0,P.useEffect)(()=>()=>{a.current!==null&&clearTimeout(a.current.timeoutId)},[]),{handleSaveGraph:o,handleLoadGraph:(0,P.useCallback)(e=>{t&&(n(`import graph from ${e}`),i(`Importing graph "${e}"…`,`info`))},[t,n,i])}}var Vn=/^[A-Za-z0-9_-]+$/,Hn=/^[A-Za-z0-9_-]+(?:\[(?:0|[1-9]\d*)?\])*$/,Un=new Set([`input`,`output`,`model`,`response`,`result`,`parameter`,`none`,`next`,`api`,`error`]);function Wn(e,t){return`properties.${e}.${t}`}function Gn(e){return e.split(`.`).every(e=>Hn.test(e))}function Kn(e){return Gn(e)}function qn(e,t={}){let n={},r=t.mode??`create`,i=e.alias.trim(),a=t.originalAlias?.trim()??``,o=e.nodeType.trim();r===`edit`?a?Vn.test(a)||(n.alias=`Use only letters, numbers, underscore, and hyphen.`):n.alias=`Original alias is required.`:i?Vn.test(i)?Un.has(i.toLowerCase())?n.alias=`"${i}" is reserved.`:t.graphData?.nodes.some(e=>e.alias.toLowerCase()===i.toLowerCase())&&(n.alias=`Node "${i}" already exists in the current graph.`):n.alias=`Use only letters, numbers, underscore, and hyphen.`:n.alias=`Alias is required.`,o&&!Vn.test(o)&&(n.nodeType=`Use only letters, numbers, underscore, and hyphen.`);for(let t of e.properties){let e=t.key.trim(),r=t.value.trim();!e&&!r||(!e&&r?n[Wn(t.id,`key`)]=`Property key is required when value is present.`:Kn(e)||(n[Wn(t.id,`key`)]=`Use a property name or dot/bracket path, for example mapping[] or config.value.`),r.includes(`'''`)&&(n[Wn(t.id,`value`)]=`Property value cannot contain '''.`))}return{valid:Object.keys(n).length===0,errors:n}}function Jn(e,t={}){let n={},r=e.trim();return r?Vn.test(r)?t.graphData&&!t.graphData.nodes.some(e=>e.alias.toLowerCase()===r.toLowerCase())&&(n.alias=`Node "${r}" is no longer available in the current graph.`):n.alias=`Use only letters, numbers, underscore, and hyphen.`:n.alias=`Alias is required.`,{valid:Object.keys(n).length===0,errors:n}}function Yn(e,t){let n={},r=e.trim(),i=t.trim();return(!r||!Vn.test(r))&&(n.sourceAlias=`Use only letters, numbers, underscore, and hyphen.`),(!i||!Vn.test(i))&&(n.targetAlias=`Use only letters, numbers, underscore, and hyphen.`),!n.sourceAlias&&!n.targetAlias&&r===i&&(n.targetAlias=`Source and target must be different nodes.`),{valid:Object.keys(n).length===0,errors:n}}function Xn(e,t){return!!e?.nodes.some(e=>e.alias.toLowerCase()===t.toLowerCase())}function Zn(e,t={}){let n={},r=e.sourceAlias.trim(),i=e.targetAlias.trim(),a=e.relation.trim();return t.connected===!1&&(n.command=`Connection disconnected. Reconnect before creating a connection.`),r?Vn.test(r)?t.graphData&&!Xn(t.graphData,r)&&(n.sourceAlias=`Node "${r}" is no longer available in the current graph.`):n.sourceAlias=`Use only letters, numbers, underscore, and hyphen.`:n.sourceAlias=`Source is required.`,i?Vn.test(i)?t.graphData&&!Xn(t.graphData,i)&&(n.targetAlias=`Node "${i}" is no longer available in the current graph.`):n.targetAlias=`Use only letters, numbers, underscore, and hyphen.`:n.targetAlias=`Target is required.`,r&&i&&r.toLowerCase()===i.toLowerCase()&&(n.targetAlias=`Source and target nodes must be different.`),a?Vn.test(a)||(n.relation=`Use only letters, numbers, underscore, and hyphen.`):n.relation=`Relation is required.`,{valid:Object.keys(n).length===0,errors:n}}function Qn(e){return e.length<=63488?{valid:!0,errors:{}}:{valid:!1,errors:{command:`The node command is too large. Shorten property values before submitting.`}}}var $n=0;function er(e=``,t=``){return $n+=1,{id:`property-row-${$n}`,key:e,value:t}}function tr(e){return{alias:e===`empty-graph`?`root`:``,nodeType:e===`empty-graph`?`Root`:``,properties:[er()],source:e}}function nr(e){let t=e.indexOf(`[`);if(t===-1)return null;let n=e.indexOf(`]`,t+1);return n===-1||e.indexOf(`[`,n+1)!==-1?null:{open:t,close:n}}function rr(e){let t=nr(e);if(!t)return e;let n=e.slice(t.open+1,t.close);return/^\d+$/.test(n)?e.slice(0,t.open+1)+n.padStart(3,`0`)+e.slice(t.close):e}function ir(e){let t=nr(e);if(!t)return e;let n=e.slice(t.open+1,t.close);return/^\d+$/.test(n)?e.slice(0,t.open+1)+e.slice(t.close):e}function ar(e){return e.slice().sort((e,t)=>{let n=rr(e.key.trim()),r=rr(t.key.trim());return!n&&!r?0:n?!r||n<r?-1:+(n>r):1})}var or=`This node contains data that cannot be safely represented in the edit form. Use the console edit command for this node.`;function sr(e){return typeof e==`object`&&!!e&&!Array.isArray(e)}function cr(e){return e===null?`null`:String(e)}function lr(e,t,n){if(!Gn(e))return!1;if(Array.isArray(t))return t.length!==0&&t.every((t,r)=>lr(`${e}[${r}]`,t,n));if(sr(t)){let r=Object.entries(t);return r.length!==0&&r.every(([t,r])=>lr(`${e}.${t}`,r,n))}let r=cr(t);return r.includes(`'''`)?!1:(n.push(er(e,r)),!0)}function ur(e){if(!Vn.test(e.alias)||e.types.length>1)return{valid:!1,formState:null,message:or};let t=Object.entries(e.properties),n=[];for(let[e,r]of t)if(!lr(e,r,n))return{valid:!1,formState:null,message:or};let r=ar(n).map(e=>({...e,key:ir(e.key)}));return{valid:!0,formState:{alias:e.alias,nodeType:e.types[0]??``,properties:r.length>0?r:[er()],source:`edit-node`},message:null}}function dr(e,t=!1){return e.properties.map(e=>({key:e.key.trim(),value:t?e.value.replace(/\r\n/g,`
`).replace(/\r/g,`
`):e.value.trim()})).filter(e=>e.key||e.value.trim())}function fr(e){let t=Qn(e);if(!t.valid)throw Error(t.errors.command)}function pr(e,t,n){if(n.includes(`
`)){e.push(`${t}='''`),e.push(n),e.push(`'''`);return}e.push(`${t}=${n}`)}function mr(e){let t=qn(e);if(!t.valid)throw Error(Object.values(t.errors)[0]??`Invalid node form state.`);let n=e.alias.trim(),r=e.nodeType.trim(),i=dr(e,!0),a=[`create node ${n}`];if(r&&a.push(`with type ${r}`),i.length>0){a.push(`with properties`);for(let e of i)pr(a,e.key,e.value)}let o=a.join(`
`);return fr(o),o}function hr(e,t){let n=t.trim(),r=qn(e,{mode:`edit`,originalAlias:n});if(!r.valid)throw Error(Object.values(r.errors)[0]??`Invalid node form state.`);let i=e.nodeType.trim(),a=dr(e,!0),o=[`update node ${n}`];if(i&&o.push(`with type ${i}`),a.length>0){o.push(`with properties`);for(let e of a)pr(o,e.key,e.value)}let s=o.join(`
`);return fr(s),s}function gr(e,t={}){let n=e.trim(),r=Jn(n,t);if(!r.valid)throw Error(Object.values(r.errors)[0]??`Invalid node alias.`);let i=`delete node ${n}`;return fr(i),i}function _r(e,t){let n=e.trim(),r=t.trim(),i=Yn(n,r);if(!i.valid)throw Error(Object.values(i.errors)[0]??`Invalid connection endpoints.`);let a=`delete connection ${n} and ${r}`;return fr(a),a}function vr(e){let t=Zn(e);if(!t.valid)throw Error(Object.values(t.errors)[0]??`Invalid connection form state.`);let n=`connect ${e.sourceAlias.trim()} to ${e.targetAlias.trim()} with ${e.relation.trim()}`;return fr(n),n}function yr(e,t){let n=ur(t);if(!n.valid||n.formState===null)throw Error(`Node "${t.alias}" cannot be pasted: its data cannot be written as a ${e} node command (an empty list or map, a value containing ''', or more than one type).`);return e===`update`?hr(n.formState,t.alias):mr(n.formState)}function br(e,t){let n=t?.nodes.some(t=>t.alias===e.node.alias)?`update`:`create`;return{verb:n,command:yr(n,e.node)}}function xr(e){return`${e} ${e===1?`node`:`nodes`} clipped to workspace`}function Sr(e){let{added:t,duplicates:n,failed:r}=e;if(t===0&&n===0&&r===0)return{message:`No selected nodes are available to clip.`,type:`info`};if(t===0&&n>0&&r===0)return{message:`All selected nodes already exist in workspace.`,type:`info`};if(t===0&&n===0&&r>0)return{message:`Failed to clip selected nodes to workspace.`,type:`error`};let i=xr(t);return n>0&&(i+=`. ${n} already existed.`),r>0&&(i+=`. ${r} failed.`),{message:i,type:r>0?`error`:`success`}}function Cr(e){return{execute(t){return e(t)}}}function wr(e,t,n){let r=[];for(let i of e.connections??[])if(!(i.source!==t||i.target!==n))for(let e of i.relations)r.push(e.type);return r}function Tr(e,t){try{let n=new Map,r=(t,r)=>{let i=`${t}\t${r}`,a=n.get(i);return a||(a=wr(e,t,r),n.set(i,a)),a},i=[],a=[],o=new Set;for(let{source:e,target:n,relation:s}of t){let t=r(e,n);if(s===void 0){if(t.length===0)continue;for(let r of t)i.push({source:e,target:n,relation:r});t.length=0}else{let r=t.indexOf(s);if(r===-1)continue;t.splice(r,1),i.push({source:e,target:n,relation:s})}let c=[e,n].sort().join(`	`);o.has(c)||(o.add(c),a.push({a:e,b:n}))}if(i.length===0)return null;let s=[];for(let{a:e,b:t}of a){s.push(_r(e,t));for(let n of r(e,t))s.push(vr({sourceAlias:e,targetAlias:t,relation:n}));for(let n of r(t,e))s.push(vr({sourceAlias:t,targetAlias:e,relation:n}))}return{commands:s,removed:i}}catch{return null}}function Er(e){let t=e[0];return e.length===1?`relation '${t.relation}' (${t.source} → ${t.target})`:e.every(e=>e.source===t.source&&e.target===t.target)?`connection ${t.source} → ${t.target}`:`${e.length} relations`}function Dr(e,t,n){let r=[];for(let i of e.connections??[]){let e=i.source===t&&i.target===n,a=i.source===n&&i.target===t;if(!(!e&&!a))for(let e of i.relations)r.push(vr({sourceAlias:i.source,targetAlias:i.target,relation:e.type}))}return r}function Or(e){if(e.length===0)return null;try{return{label:`delete ${Er(e)}`,inverseCommands:e.map(({source:e,target:t,relation:n})=>vr({sourceAlias:e,targetAlias:t,relation:n}))}}catch{return null}}function kr(e){let t=ur(e);if(!t.valid||t.formState===null)return null;try{return{label:`edit node ${e.alias}`,inverseCommands:[hr(t.formState,e.alias)]}}catch{return null}}function Ar(e){try{return{label:`create node ${e}`,inverseCommands:[gr(e)]}}catch{return null}}function jr(e,t,n){try{let r=Dr(e,t,n);return{label:`create connection ${t} → ${n}`,inverseCommands:[_r(t,n),...r]}}catch{return null}}function Mr(e,t){let n=ur(t);if(!n.valid||n.formState===null)return null;try{let r=[mr(n.formState)];for(let n of e.connections??[])if(!(n.source!==t.alias&&n.target!==t.alias))for(let e of n.relations)r.push(vr({sourceAlias:n.source,targetAlias:n.target,relation:e.type}));return{label:`delete node ${t.alias}`,inverseCommands:r}}catch{return null}}var Nr=2500;function Pr({bus:e,connected:t,sendRawText:n,addToast:r}){let i=(0,P.useRef)([]),a=(0,P.useRef)(0),o=(0,P.useCallback)(()=>{i.current=[]},[]);(0,P.useEffect)(()=>{t||o()},[o,t]),(0,P.useEffect)(()=>e.on(`graph.mutation`,e=>{e.mutationType===`import-graph`&&o()}),[e,o]);let s=(0,P.useCallback)(e=>{if(e===null||e.inverseCommands.length===0)return null;let t=++a.current;return i.current=[...i.current.slice(-19),{...e,id:t}],t},[]),c=(0,P.useRef)(!1),l=(0,P.useCallback)(()=>new Promise(t=>{let n=!1,r=()=>{n||(n=!0,i(),clearTimeout(a),t())},i=e.on(`graph.mutation`,r),a=setTimeout(r,Nr)}),[e]),u=(0,P.useRef)([]),d=(0,P.useCallback)(e=>{e.length!==0&&(u.current.push(e),!c.current&&(c.current=!0,(async()=>{try{for(;;){let e=u.current.shift();if(!e)break;for(let t of e){if(!n(t)){r(`Could not send the command because the WebSocket is not open.`,`error`),u.current=[];return}await l()}}}finally{c.current=!1}})()))},[r,n,l]),f=(0,P.useCallback)(e=>{if(c.current){r(`A graph edit is already in progress.`,`info`);return}let t=i.current,a=t[t.length-1];if(!a){r(`Nothing to undo.`,`info`);return}if(e!==void 0&&a.id!==e){r(`Newer changes exist — press Ctrl+Z to undo them in order.`,`info`);return}i.current=t.slice(0,-1),c.current=!0,(async()=>{try{for(let e of a.inverseCommands){if(!n(e)){r(`Could not send the undo command because the WebSocket is not open.`,`error`),o();return}await l()}r(`Undo: ${a.label}`,`success`)}finally{c.current=!1}})()},[r,o,n,l]);return{push:s,undoLast:(0,P.useCallback)(()=>f(void 0),[f]),undoEntry:f,runCommands:d,hasEntries:(0,P.useCallback)(()=>i.current.length>0,[]),clear:o}}function Fr(e){return e.trim().toLowerCase()}function Ir(e,t){if(!t.alias||t.action!==null&&t.action!==`delete-node`)return null;let n=e.aliases.find(n=>Fr(n)===Fr(t.alias)&&e.results[n]===void 0);return n?{...e,results:{...e.results,[n]:t.status===`accepted`?`success`:`error`}}:null}function Lr(e){return e.aliases.every(t=>e.results[t]!==void 0)}function Rr(e){let t=Object.values(e.results).filter(e=>e===`success`).length,n=Object.values(e.results).filter(e=>e===`error`).length,r=`${t} selected ${t===1?`node`:`nodes`} deleted.`;return t===0&&n>0?{message:`Failed to delete selected nodes.`,type:`error`}:n>0?{message:`${r} ${n} failed.`,type:`error`}:{message:r,type:`success`}}var zr={toastContainer:`_toastContainer_1rn7u_1`,toast:`_toast_1rn7u_1`,slideIn:`_slideIn_1rn7u_1`,success:`_success_1rn7u_36`,error:`_error_1rn7u_40`,info:`_info_1rn7u_44`,toastIcon:`_toastIcon_1rn7u_48`,toastMessage:`_toastMessage_1rn7u_53`,toastAction:`_toastAction_1rn7u_59`},Br=({toasts:e,onRemove:t})=>e.length===0?null:(0,z.jsx)(`div`,{className:zr.toastContainer,children:e.map(e=>(0,z.jsxs)(`div`,{className:`${zr.toast} ${zr[e.type]}`,onClick:()=>t(e.id),children:[(0,z.jsxs)(`span`,{className:zr.toastIcon,children:[e.type===`success`&&`✅`,e.type===`error`&&`❌`,e.type===`info`&&`ℹ️`]}),(0,z.jsx)(`span`,{className:zr.toastMessage,children:e.message}),e.action&&(0,z.jsx)(`button`,{type:`button`,className:zr.toastAction,onClick:n=>{n.stopPropagation(),e.action?.onClick(),t(e.id)},children:e.action.label})]},e.id))}),Vr={container:`_container_9dbh2_3`,trigger:`_trigger_9dbh2_7`,chevron:`_chevron_9dbh2_37`,chevronOpen:`_chevronOpen_9dbh2_43`,dot:`_dot_9dbh2_49`,dotIdle:`_dotIdle_9dbh2_56`,dotConnecting:`_dotConnecting_9dbh2_57`,pulse:`_pulse_9dbh2_1`,dotConnected:`_dotConnected_9dbh2_58`,dotPartial:`_dotPartial_9dbh2_59`,dropdown:`_dropdown_9dbh2_65`,fadeIn:`_fadeIn_9dbh2_1`};function Hr({label:e,dotStatus:t,children:n}){let[r,i]=(0,P.useState)(!1),a=(0,P.useRef)(null),o=(0,P.useCallback)(()=>i(!1),[]);(0,P.useEffect)(()=>{if(!r)return;let e=e=>{a.current&&!a.current.contains(e.target)&&i(!1)};return document.addEventListener(`mousedown`,e),()=>document.removeEventListener(`mousedown`,e)},[r]);let s=e=>{e.key===`Escape`&&(i(!1),a.current?.querySelector(`button[aria-haspopup]`)?.focus())},c=t===`connected`?Vr.dotConnected:t===`connecting`?Vr.dotConnecting:t===`partial`?Vr.dotPartial:t===`idle`?Vr.dotIdle:void 0;return(0,z.jsxs)(`div`,{className:Vr.container,ref:a,onKeyDown:s,children:[(0,z.jsxs)(`button`,{className:Vr.trigger,onClick:()=>i(e=>!e),"aria-haspopup":`true`,"aria-expanded":r,children:[t!==void 0&&(0,z.jsx)(`span`,{className:`${Vr.dot} ${c??``}`,"aria-hidden":`true`}),(0,z.jsx)(`span`,{children:e}),(0,z.jsx)(`span`,{className:`${Vr.chevron} ${r?Vr.chevronOpen:``}`,"aria-hidden":`true`,children:`▾`})]}),r&&(0,z.jsx)(`div`,{className:Vr.dropdown,role:`menu`,children:typeof n==`function`?n(o):n})]})}var B={panel:`_panel_1ws0d_1`,section:`_section_1ws0d_7`,sectionHeader:`_sectionHeader_1ws0d_20`,sessionHeaderRow:`_sessionHeaderRow_1ws0d_29`,iconButton:`_iconButton_1ws0d_36`,copyButton:`_copyButton_1ws0d_63`,copyButtonCopied:`_copyButtonCopied_1ws0d_95`,relationshipRow:`_relationshipRow_1ws0d_102`,subscriberRow:`_subscriberRow_1ws0d_103`,sessionId:`_sessionId_1ws0d_114`,statusDot:`_statusDot_1ws0d_127`,metaText:`_metaText_1ws0d_136`,emptyMessage:`_emptyMessage_1ws0d_137`,subscriberList:`_subscriberList_1ws0d_148`,subscribeForm:`_subscribeForm_1ws0d_157`,subscribeInput:`_subscribeInput_1ws0d_164`,subscribeButton:`_subscribeButton_1ws0d_189`,resetButton:`_resetButton_1ws0d_190`,unsubscribeButton:`_unsubscribeButton_1ws0d_218`,actionsRow:`_actionsRow_1ws0d_241`,errorMessage:`_errorMessage_1ws0d_261`,infoMessage:`_infoMessage_1ws0d_262`};function Ur({copied:e}){return e?(0,z.jsx)(`svg`,{viewBox:`0 0 16 16`,"aria-hidden":`true`,focusable:`false`,children:(0,z.jsx)(`path`,{d:`M6.2 11.4 2.9 8.1l1.1-1.1 2.2 2.2 5.8-5.8 1.1 1.1-6.9 6.9Z`})}):(0,z.jsxs)(`svg`,{viewBox:`0 0 16 16`,"aria-hidden":`true`,focusable:`false`,children:[(0,z.jsx)(`path`,{d:`M5 2.5A1.5 1.5 0 0 1 6.5 1h6A1.5 1.5 0 0 1 14 2.5v6A1.5 1.5 0 0 1 12.5 10H11V8.7h1.5a.2.2 0 0 0 .2-.2v-6a.2.2 0 0 0-.2-.2h-6a.2.2 0 0 0-.2.2V4H5V2.5Z`}),(0,z.jsx)(`path`,{d:`M2 6.5A1.5 1.5 0 0 1 3.5 5h6A1.5 1.5 0 0 1 11 6.5v7A1.5 1.5 0 0 1 9.5 15h-6A1.5 1.5 0 0 1 2 13.5v-7Zm1.5-.2a.2.2 0 0 0-.2.2v7a.2.2 0 0 0 .2.2h6a.2.2 0 0 0 .2-.2v-7a.2.2 0 0 0-.2-.2h-6Z`})]})}function Wr(e){return e.connected?e.state.pendingCommand===`refresh`||e.state.loading?`connecting`:e.state.sessionId?`connected`:`partial`:`idle`}function Gr({controller:e}){let{state:t}=e,[n,r]=(0,P.useState)(!1),[i,a]=(0,P.useState)(``),[o,s]=(0,P.useState)(null),[c,l]=(0,P.useState)(null),u=(0,P.useRef)(null),d=e.canReset;(0,P.useEffect)(()=>()=>{u.current!==null&&window.clearTimeout(u.current)},[]),(0,P.useEffect)(()=>{t.subscribedTo&&t.pendingCommand!==`subscribe`&&(r(!1),a(``))},[t.pendingCommand,t.subscribedTo]);let f=t=>{t.preventDefault(),e.subscribeToSession(i)},p=async e=>{try{await navigator.clipboard.writeText(e),s(null),l(e),u.current!==null&&window.clearTimeout(u.current),u.current=window.setTimeout(()=>{l(null),u.current=null},1400)}catch{l(null),s(`Could not copy the session ID. Please copy it manually.`)}};return e.connected?(0,z.jsxs)(`div`,{className:B.panel,children:[(0,z.jsxs)(`section`,{className:B.section,children:[(0,z.jsx)(`div`,{className:B.sectionHeader,children:`This session`}),(0,z.jsxs)(`div`,{className:B.sessionHeaderRow,children:[t.sessionId?(0,z.jsx)(`code`,{className:B.sessionId,title:t.sessionId,children:t.sessionId}):(0,z.jsx)(`p`,{className:B.emptyMessage,children:`Session details are not loaded yet.`}),t.sessionId&&(0,z.jsx)(`button`,{type:`button`,className:`${B.copyButton} ${c===t.sessionId?B.copyButtonCopied:``}`,onClick:()=>void p(t.sessionId),"aria-label":`Copy session ID`,title:c===t.sessionId?`Copied`:`Copy session ID`,children:(0,z.jsx)(Ur,{copied:c===t.sessionId})}),e.canSubscribe&&(0,z.jsx)(`button`,{type:`button`,className:B.iconButton,onClick:()=>{e.clearMessage(),r(e=>!e)},"aria-expanded":n,"aria-label":n?`Close subscribe form`:`Subscribe to another session`,title:n?`Close subscribe form`:`Subscribe to another session`,children:n?`×`:`+`})]}),t.startedSince&&(0,z.jsxs)(`div`,{className:B.metaText,children:[`Started since `,t.startedSince]}),n&&(0,z.jsxs)(`form`,{className:B.subscribeForm,onSubmit:f,children:[(0,z.jsx)(`input`,{className:B.subscribeInput,value:i,onChange:t=>{a(t.target.value),e.clearMessage()},placeholder:`ws-123456-1`,autoComplete:`off`,disabled:!e.canSubscribe}),(0,z.jsx)(`button`,{type:`submit`,className:B.subscribeButton,disabled:!e.canSubscribe||i.trim().length===0,children:t.pendingCommand===`subscribe`?`...`:`Subscribe`})]})]}),t.subscribedTo&&(0,z.jsxs)(`section`,{className:B.section,children:[(0,z.jsx)(`div`,{className:B.sectionHeader,children:`Subscribed to`}),(0,z.jsxs)(`div`,{className:B.relationshipRow,children:[(0,z.jsx)(`span`,{className:B.statusDot,"aria-hidden":`true`}),(0,z.jsx)(`code`,{className:B.sessionId,title:t.subscribedTo,children:t.subscribedTo}),(0,z.jsx)(`button`,{type:`button`,className:B.unsubscribeButton,onClick:e.unsubscribe,disabled:!e.canUnsubscribe,children:t.pendingCommand===`unsubscribe`?`...`:`Unsubscribe`})]})]}),t.subscribers.length>0&&(0,z.jsxs)(`section`,{className:B.section,children:[(0,z.jsx)(`div`,{className:B.sectionHeader,children:`Subscribers`}),(0,z.jsx)(`ul`,{className:B.subscriberList,children:t.subscribers.map(e=>(0,z.jsxs)(`li`,{className:B.subscriberRow,children:[(0,z.jsx)(`span`,{className:B.statusDot,"aria-hidden":`true`}),(0,z.jsx)(`code`,{className:B.sessionId,title:e,children:e})]},e))})]}),d&&(0,z.jsx)(`div`,{className:B.actionsRow,children:(0,z.jsx)(`button`,{type:`button`,className:B.resetButton,onClick:e.resetSession,disabled:!e.canReset,children:t.pendingCommand===`reset`?`Resetting...`:`Reset Session`})}),t.error&&(0,z.jsx)(`div`,{className:B.errorMessage,role:`alert`,children:t.error}),o&&(0,z.jsx)(`div`,{className:B.errorMessage,role:`alert`,children:o})]}):(0,z.jsx)(`div`,{className:B.panel,children:(0,z.jsx)(`p`,{className:B.emptyMessage,children:`Connect Minigraph to view session details.`})})}function Kr({controller:e}){return(0,z.jsx)(Hr,{label:`Session`,dotStatus:Wr(e),children:(0,z.jsx)(Gr,{controller:e})})}var V={nav:`_nav_7x22x_3`,menuList:`_menuList_7x22x_11`,menuItem:`_menuItem_7x22x_19`,menuButton:`_menuButton_7x22x_47`,menuSection:`_menuSection_7x22x_57`,toolRow:`_toolRow_7x22x_72`,toolLink:`_toolLink_7x22x_83`,toolLinkActive:`_toolLinkActive_7x22x_108`,toolDot:`_toolDot_7x22x_115`,toolDotIdle:`_toolDotIdle_7x22x_122`,toolDotConnecting:`_toolDotConnecting_7x22x_123`,pulse:`_pulse_7x22x_1`,toolDotConnected:`_toolDotConnected_7x22x_124`,connectAllRow:`_connectAllRow_7x22x_128`,connectAllBtn:`_connectAllBtn_7x22x_134`,connectAllBtnStop:`_connectAllBtnStop_7x22x_158`,toolConnectBtn:`_toolConnectBtn_7x22x_170`,toolConnectBtnStop:`_toolConnectBtnStop_7x22x_196`,externalIcon:`_externalIcon_7x22x_208`};function qr(e){return e.every(e=>e===`connected`)?`connected`:e.every(e=>e===`idle`)?`idle`:e.some(e=>e===`connecting`)?`connecting`:`partial`}function Jr(e){return e===`connected`?`connected`:e===`connecting`?`connecting`:`idle`}var Yr=[{href:`/info`,label:`Info`},{href:`/info/lib`,label:`Libraries`},{href:`/info/routes`,label:`Services`},{href:`/health`,label:`Health`},{href:`/env`,label:`Environment`},{href:`http://localhost:8085/api/ws/json`,label:`Legacy JSON`},{href:`http://localhost:8085/api/ws/graph`,label:`Legacy Graph`}];function Xr({addToast:e,sessionCollaboration:t,onPackageGraphs:n,onEditSchema:r}){let i=ge(),a=ue.map(e=>i.getSlot(e.wsPath).phase),o=qr(a),s=a.every(e=>e===`connected`),c=a.some(e=>e===`connecting`);function l(){ue.forEach(t=>{i.getSlot(t.wsPath).phase===`idle`&&i.connect(t.wsPath,e)})}function u(){ue.forEach(e=>{let{phase:t}=i.getSlot(e.wsPath);(t===`connected`||t===`connecting`)&&i.disconnect(e.wsPath)})}return(0,z.jsxs)(`nav`,{className:V.nav,"aria-label":`Main navigation`,children:[t&&(0,z.jsx)(Kr,{controller:t}),(0,z.jsx)(Hr,{label:`Tools`,dotStatus:o,children:t=>(0,z.jsxs)(z.Fragment,{children:[(0,z.jsx)(`div`,{className:V.connectAllRow,children:(0,z.jsx)(`button`,{className:`${V.connectAllBtn} ${s?V.connectAllBtnStop:``}`,onClick:s?u:l,disabled:c,"aria-label":c?`Connecting…`:s?`Disconnect all WebSockets`:`Connect all WebSockets`,children:c?`Connecting…`:s?`Disconnect All`:`Connect All`})}),(0,z.jsx)(`ul`,{className:V.menuList,role:`none`,children:ue.map(t=>{let{phase:n}=i.getSlot(t.wsPath),r=Jr(n),a=n===`connected`,o=n===`connecting`,s=r===`connected`?V.toolDotConnected:r===`connecting`?V.toolDotConnecting:V.toolDotIdle;return(0,z.jsxs)(`li`,{role:`none`,className:V.toolRow,children:[(0,z.jsxs)(w,{to:t.path,role:`menuitem`,className:({isActive:e})=>`${V.toolLink} ${e?V.toolLinkActive:``}`,children:[(0,z.jsx)(`span`,{className:`${V.toolDot} ${s}`,"aria-hidden":`true`}),(0,z.jsx)(`span`,{className:V.toolLabel,children:t.label})]}),(0,z.jsx)(`button`,{className:`${V.toolConnectBtn} ${a?V.toolConnectBtnStop:``}`,onClick:()=>a||o?i.disconnect(t.wsPath):i.connect(t.wsPath,e),disabled:o,"aria-label":o?`Connecting…`:a?`Disconnect ${t.label}`:`Connect ${t.label}`,title:o?`Connecting…`:de(t.wsPath),children:o?`…`:a?`Stop`:`Start`})]},t.path)})}),(r||n)&&(0,z.jsxs)(`ul`,{className:`${V.menuList} ${V.menuSection}`,role:`none`,children:[r&&(0,z.jsx)(`li`,{role:`none`,children:(0,z.jsx)(`button`,{type:`button`,role:`menuitem`,className:`${V.menuItem} ${V.menuButton}`,onClick:()=>{t(),r()},title:`Declare the graph's request and response contract - the schema on the root and end nodes`,children:`📐 Graph schema`})}),n&&(0,z.jsx)(`li`,{role:`none`,children:(0,z.jsx)(`button`,{type:`button`,role:`menuitem`,className:`${V.menuItem} ${V.menuButton}`,onClick:()=>{t(),n()},title:`Pack graph models into a graph set (<set>.pack), or inspect one`,children:`📦 Graph set packaging`})})]})]})}),(0,z.jsx)(Hr,{label:`Quick Links`,children:(0,z.jsx)(`ul`,{className:V.menuList,role:`none`,children:Yr.map(e=>(0,z.jsx)(`li`,{role:`none`,children:(0,z.jsxs)(`a`,{href:e.href,role:`menuitem`,className:V.menuItem,target:`_blank`,rel:`noopener noreferrer`,children:[e.label,(0,z.jsx)(`span`,{className:V.externalIcon,"aria-hidden":`true`,children:`↗`})]})},e.href))})})]})}var Zr={saveBtn:`_saveBtn_ek34s_3`,saveBtnSaved:`_saveBtnSaved_ek34s_25`,saveBtnLabel:`_saveBtnLabel_ek34s_35`,saveForm:`_saveForm_ek34s_50`,saveInput:`_saveInput_ek34s_56`,saveInputWarn:`_saveInputWarn_ek34s_72`,saveWarnLabel:`_saveWarnLabel_ek34s_76`,saveActionBtn:`_saveActionBtn_ek34s_82`};function Qr(e){return e?`Saved: ${e}`:`Save Graph`}function $r({disabled:e,defaultName:t,savedName:n=null,onSave:r,nameExists:i,connected:a=!1}){let[o,s]=(0,P.useState)(!1),[c,l]=(0,P.useState)(``),u=(0,P.useRef)(null),d=(0,P.useRef)(null),f=(0,P.useRef)(!1),p=(0,P.useCallback)(()=>{l(t),s(!0)},[t]),m=(0,P.useCallback)(()=>{f.current=!0,s(!1),l(``)},[]),h=(0,P.useCallback)(()=>{let e=c.trim();e&&(r(e),f.current=!0,s(!1),l(``))},[c,r]),g=(0,P.useCallback)(e=>{e.key===`Enter`&&(e.preventDefault(),h()),e.key===`Escape`&&(e.preventDefault(),m())},[h,m]);return(0,P.useEffect)(()=>{o?u.current?.focus():f.current&&(f.current=!1,d.current?.focus())},[o]),o?(0,z.jsxs)(`div`,{className:Zr.saveForm,children:[(0,z.jsx)(`input`,{ref:u,className:`${Zr.saveInput}${i?.(c.trim())?` ${Zr.saveInputWarn}`:``}`,type:`text`,value:c,onChange:e=>l(e.target.value),onKeyDown:g,placeholder:`Enter a name…`,"aria-label":`Graph save name`,maxLength:80}),i?.(c.trim())&&(0,z.jsx)(`span`,{className:Zr.saveWarnLabel,role:`status`,children:`Overwrite?`}),(0,z.jsx)(`button`,{className:Zr.saveActionBtn,onClick:h,disabled:!c.trim(),"aria-label":`Confirm save`,children:`✅`}),(0,z.jsx)(`button`,{className:Zr.saveActionBtn,onClick:m,"aria-label":`Cancel save`,children:`❌`})]}):(0,z.jsxs)(`button`,{ref:d,className:`${Zr.saveBtn}${n?` ${Zr.saveBtnSaved}`:``}`,onClick:p,disabled:e||!a,title:e?`No graph loaded`:a?n?`Graph saved as ${n}. Click to save again`:`Export graph snapshot to server and save bookmark`:`Connect first to save`,"aria-label":n?`Graph saved as ${n}. Save again`:`Save graph snapshot`,children:[(0,z.jsx)(`span`,{"aria-hidden":`true`,children:n?`✅`:`💾`}),(0,z.jsx)(`span`,{className:Zr.saveBtnLabel,children:Qr(n)})]})}var ei={empty:`_empty_tpeii_3`,hint:`_hint_tpeii_12`,list:`_list_tpeii_21`,row:`_row_tpeii_31`,rowInfo:`_rowInfo_tpeii_50`,rowName:`_rowName_tpeii_58`,rowMeta:`_rowMeta_tpeii_67`,rowActions:`_rowActions_tpeii_78`,loadBtn:`_loadBtn_tpeii_84`,deleteBtn:`_deleteBtn_tpeii_85`};function ti({savedGraphs:e,onLoad:t,onDelete:n,connected:r}){return(0,z.jsx)(Hr,{label:e.length>0?`Load Graph (${e.length})`:`Load Graph`,children:e.length===0?(0,z.jsx)(`p`,{className:ei.empty,children:`No saved graphs yet.`}):(0,z.jsxs)(z.Fragment,{children:[!r&&(0,z.jsx)(`p`,{className:ei.hint,children:`Connect to load a graph`}),(0,z.jsx)(`ul`,{className:ei.list,role:`list`,children:e.map(e=>(0,z.jsxs)(`li`,{className:ei.row,children:[(0,z.jsxs)(`div`,{className:ei.rowInfo,children:[(0,z.jsx)(`span`,{className:ei.rowName,title:e.name,children:e.name}),(0,z.jsx)(`span`,{className:ei.rowMeta,children:new Date(e.savedAt).toLocaleString()})]}),(0,z.jsxs)(`div`,{className:ei.rowActions,children:[(0,z.jsx)(`button`,{className:ei.loadBtn,onClick:()=>t(e.name),disabled:!r,title:r?`Run: import graph from ${e.name}`:`Connect to the playground first`,"aria-label":`Load graph ${e.name}`,children:`Load`}),(0,z.jsx)(`button`,{className:ei.deleteBtn,onClick:()=>n(e.name),title:`Remove "${e.name}" from local storage`,"aria-label":`Delete saved graph ${e.name}`,children:`Delete`})]})]},e.name))})]})})}var ni={payloadRoot:`_payloadRoot_6u47x_2`,labelRow:`_labelRow_6u47x_10`,label:`_label_6u47x_10`,payloadControls:`_payloadControls_6u47x_26`,charCounter:`_charCounter_6u47x_32`,typeIndicator:`_typeIndicator_6u47x_38`,validationIcon:`_validationIcon_6u47x_49`,formatButton:`_formatButton_6u47x_53`,uploadButton:`_uploadButton_6u47x_67`,textarea:`_textarea_6u47x_82`,textareaError:`_textareaError_6u47x_107`,errorMessage:`_errorMessage_6u47x_109`,sampleButtonsRow:`_sampleButtonsRow_6u47x_117`,sampleButtons:`_sampleButtons_6u47x_117`,sampleLabel:`_sampleLabel_6u47x_130`,sampleGroup:`_sampleGroup_6u47x_136`,sampleGroupLabel:`_sampleGroupLabel_6u47x_143`,sampleButton:`_sampleButton_6u47x_117`};function ri({onLoad:e}){let t=Object.keys(R).filter(e=>e.startsWith(`json_`)),n=Object.keys(R).filter(e=>e.startsWith(`xml_`)),r=e=>e.replace(/^(json|xml)_/,``).replace(/_/g,` `);return(0,z.jsxs)(`div`,{className:ni.sampleButtons,children:[(0,z.jsx)(`span`,{className:ni.sampleLabel,children:`Quick load:`}),(0,z.jsxs)(`div`,{className:ni.sampleGroup,children:[(0,z.jsx)(`span`,{className:ni.sampleGroupLabel,children:`JSON:`}),t.map(t=>(0,z.jsx)(`button`,{className:ni.sampleButton,onClick:()=>e(R[t]),children:r(t)},t))]}),(0,z.jsxs)(`div`,{className:ni.sampleGroup,children:[(0,z.jsx)(`span`,{className:ni.sampleGroupLabel,children:`XML:`}),n.map(t=>(0,z.jsx)(`button`,{className:ni.sampleButton,onClick:()=>e(R[t]),children:r(t)},t))]})]})}function ii({payload:e,onChange:t,validation:n,onFormat:r,onUpload:i}){return(0,z.jsxs)(`div`,{className:ni.payloadRoot,children:[(0,z.jsxs)(`div`,{className:ni.labelRow,children:[(0,z.jsx)(`label`,{htmlFor:`payload`,className:ni.label,children:`JSON/XML Payload`}),(0,z.jsxs)(`div`,{className:ni.payloadControls,children:[(0,z.jsxs)(`span`,{className:ni.charCounter,children:[`size: `,e.length]}),e&&n.type&&(0,z.jsx)(`span`,{className:ni.typeIndicator,children:n.type.toUpperCase()}),e&&(0,z.jsx)(`span`,{className:ni.validationIcon,children:n.valid?`✅`:`❌`}),(0,z.jsx)(`button`,{className:ni.formatButton,onClick:r,disabled:!e||n.type!==`json`,title:n.type===`xml`?`Format only available for JSON`:`Format JSON`,children:`Format`}),i!==void 0&&(0,z.jsx)(`button`,{className:ni.uploadButton,onClick:i,disabled:!e||!n.valid||n.type!==`json`,title:`Upload JSON payload to current session via REST`,children:`Upload`})]})]}),(0,z.jsx)(`textarea`,{id:`payload`,className:`${ni.textarea} ${n.valid?``:ni.textareaError}`,placeholder:`Paste your JSON/XML payload here`,value:e,onChange:e=>t(e.target.value)}),!n.valid&&(0,z.jsx)(`div`,{className:ni.errorMessage,children:n.error}),(0,z.jsx)(`div`,{className:ni.sampleButtonsRow,children:(0,z.jsx)(ri,{onLoad:t})})]})}var ai={Root:{icon:`🚀`,label:`Root`},End:{icon:`🏁`,label:`End`},Fetcher:{icon:`🌐`,label:`Fetcher`},mapper:{icon:`🗺️`,label:`Mapper`},Math:{icon:`🔢`,label:`Math`},JavaScript:{icon:`📜`,label:`JavaScript`},Provider:{icon:`🔌`,label:`Provider`},Dictionary:{icon:`📖`,label:`Dictionary`},Join:{icon:`🔀`,label:`Join`},Extension:{icon:`🧩`,label:`Extension`},Island:{icon:`🏝️`,label:`Island`},Decision:{icon:`❓`,label:`Decision`},Suspend:{icon:`⏸️`,label:`Suspend`},Resume:{icon:`▶️`,label:`Resume`},Suspensible:{icon:`⏯️`,label:`Suspensible`}},oi={boxSizing:`border-box`,borderRadius:`8px`,borderWidth:`1.5px`,borderStyle:`solid`,background:`var(--bg-secondary, #1e1e2e)`,color:`var(--text-primary, #cdd6f4)`,fontSize:`0.75rem`,boxShadow:`0 2px 8px rgba(0,0,0,0.45)`,overflow:`visible`,padding:0},si={Root:`#15803d`,End:`#dc2626`,Fetcher:`#2563eb`,mapper:`#ea580c`,Math:`#a16207`,JavaScript:`#7e22ce`,Provider:`#be185d`,Dictionary:`#0e7490`,Join:`#65a30d`,Extension:`#4338ca`,Island:`#475569`,Decision:`#b45309`,Suspend:`#0d9488`,Resume:`#0284c7`,Suspensible:`#c026d3`},ci=`#6c7086`;function li(e){return ai[e]??{icon:`📦`,label:e}}function ui(e){return si[e]??ci}function di(e){let t=si[e]??ci;return{...oi,borderColor:t,"--node-accent":t}}var fi={content:`_content_1g9w1_8`,header:`_header_1g9w1_24`,icon:`_icon_1g9w1_44`,alias:`_alias_1g9w1_49`,badge:`_badge_1g9w1_55`,body:`_body_1g9w1_67`,bodyPeek:`_bodyPeek_1g9w1_77`,row:`_row_1g9w1_85`,label:`_label_1g9w1_98`,value:`_value_1g9w1_104`,edgeHandle:`_edgeHandle_1g9w1_118`,authoringRing:`_authoringRing_1g9w1_134`,authoringTargetOverlay:`_authoringTargetOverlay_1g9w1_197`,authoringTargetOverlayActive:`_authoringTargetOverlayActive_1g9w1_213`};function pi({label:e,value:t}){return(0,z.jsxs)(`div`,{className:fi.row,children:[(0,z.jsx)(`span`,{className:fi.label,children:e}),(0,z.jsx)(`span`,{className:fi.value,title:t,children:t})]})}function mi({properties:e}){let t=Object.entries(e).filter(([,e])=>e!=null);return t.length===0?null:(0,z.jsx)(z.Fragment,{children:t.map(([e,t])=>Array.isArray(t)?t.map((t,n)=>{let r=typeof t==`string`?t:JSON.stringify(t);return(0,z.jsx)(pi,{label:n===0?e:``,value:r},`${e}-${n}`)}):(0,z.jsx)(pi,{label:e,value:typeof t==`string`?t:JSON.stringify(t)},e))})}function hi({alias:e,nodeType:t,properties:n,compact:r=!1}){let i=li(t);return(0,z.jsx)(P.Fragment,{children:(0,z.jsxs)(`div`,{className:fi.content,children:[(0,z.jsxs)(`div`,{className:fi.header,children:[(0,z.jsx)(`span`,{className:fi.icon,children:i.icon}),(0,z.jsx)(`span`,{className:fi.alias,children:e}),(0,z.jsx)(`span`,{className:fi.badge,children:i.label})]}),(0,z.jsx)(`div`,{className:r?`${fi.body} ${fi.bodyPeek}`:fi.body,children:(0,z.jsx)(mi,{properties:n})})]})})}function gi({id:e,data:t,isConnectable:n,selected:r}){let[i,a]=(0,P.useState)(!1),o=t.supportsConnectionAuthoring&&!i,c=p(),l=o&&c.inProgress&&c.fromNode?.id!==e;return(0,z.jsxs)(z.Fragment,{children:[(0,z.jsx)(y,{minWidth:180,minHeight:t.minHeight,isVisible:r,onResizeStart:()=>a(!0),onResizeEnd:()=>a(!1)}),t.targetHandles.map(({id:e,offset:t})=>(0,z.jsx)(s,{id:e,type:`target`,position:_.Left,isConnectable:n,className:fi.edgeHandle,style:{top:`calc(50% + ${t}px)`}},e)),t.backSourceHandles.map(({id:e,offset:t})=>(0,z.jsx)(s,{id:e,type:`source`,position:_.Left,isConnectable:n,className:fi.edgeHandle,style:{top:`calc(50% + ${t}px)`}},e)),(0,z.jsx)(hi,{alias:t.alias,nodeType:t.nodeType,properties:t.properties,compact:t.compact}),o&&(0,z.jsxs)(z.Fragment,{children:[(0,z.jsx)(s,{id:`authoring-source`,type:`source`,position:_.Right,isConnectable:n,isConnectableEnd:!1,className:fi.authoringRing}),(0,z.jsx)(s,{id:`authoring-target`,type:`target`,position:_.Left,isConnectable:n,isConnectableStart:!1,className:l?`${fi.authoringTargetOverlay} ${fi.authoringTargetOverlayActive}`:fi.authoringTargetOverlay})]}),t.sourceHandles.map(({id:e,offset:t})=>(0,z.jsx)(s,{id:e,type:`source`,position:_.Right,isConnectable:n,className:fi.edgeHandle,style:{top:`calc(50% + ${t}px)`}},e)),t.backTargetHandles.map(({id:e,offset:t})=>(0,z.jsx)(s,{id:e,type:`target`,position:_.Right,isConnectable:n,className:fi.edgeHandle,style:{top:`calc(50% + ${t}px)`}},e))]})}var _i={Root:gi,End:gi,Fetcher:gi,mapper:gi,Math:gi,JavaScript:gi,Provider:gi,Dictionary:gi,Join:gi,Extension:gi,Island:gi,Decision:gi,default:gi},vi={graphWrapper:`_graphWrapper_i2utu_15`,graphSurface:`_graphSurface_i2utu_24`,empty:`_empty_i2utu_30`,emptyIcon:`_emptyIcon_i2utu_43`,emptyCreateButton:`_emptyCreateButton_i2utu_48`,emptyHint:`_emptyHint_i2utu_70`,emptyActions:`_emptyActions_i2utu_75`,emptySecondaryButton:`_emptySecondaryButton_i2utu_87`,refreshingOverlay:`_refreshingOverlay_i2utu_169`,clipboardDropOverlay:`_clipboardDropOverlay_i2utu_181`,fileDropOverlay:`_fileDropOverlay_i2utu_195`,fileDropMessage:`_fileDropMessage_i2utu_208`,clipboardDropMessage:`_clipboardDropMessage_i2utu_219`,refreshingSpinner:`_refreshingSpinner_i2utu_234`,graphRefreshSpin:`_graphRefreshSpin_i2utu_1`,connectBanner:`_connectBanner_i2utu_248`,connectBannerCancel:`_connectBannerCancel_i2utu_268`,detailToggleIcon:`_detailToggleIcon_i2utu_307`,detailToggleHeader:`_detailToggleHeader_i2utu_318`,detailToggleLine:`_detailToggleLine_i2utu_325`},yi=class extends P.Component{constructor(...e){super(...e),this.state={caughtError:null}}static getDerivedStateFromError(e){return{caughtError:e instanceof Error?e.message:String(e)}}componentDidCatch(e,t){let n=e instanceof Error?e.message:String(e);console.error(`[GraphView] Render error:`,n,t.componentStack),this.props.onRenderError?.(`Graph render failed: ${n}`)}render(){return this.state.caughtError?(0,z.jsxs)(`div`,{className:vi.empty,children:[(0,z.jsx)(`span`,{className:vi.emptyIcon,children:`⚠️`}),(0,z.jsx)(`span`,{children:`Graph could not be rendered.`}),(0,z.jsx)(`span`,{children:this.state.caughtError})]}):this.props.children}},bi=[`fetch`,`details`,`ext-call`,`mapping`,`compute`,`calculate`,`evaluate`,`fork`,`join`,`one`,`two`,`three`,`more`,`done`,`complete`,`finish`,`positive`,`negative`],xi={fetch:`#0369a1`,details:`#0369a1`,"ext-call":`#0369a1`,mapping:`#b45309`,compute:`#b45309`,calculate:`#b45309`,evaluate:`#b45309`,fork:`#7e22ce`,join:`#7e22ce`,one:`#7e22ce`,two:`#6d28d9`,three:`#5b21b6`,more:`#4c1d95`,done:`#15803d`,complete:`#15803d`,finish:`#15803d`,positive:`#15803d`,negative:`#b91c1c`},Si=[`#0369a1`,`#15803d`,`#b45309`,`#7e22ce`,`#b91c1c`,`#0f766e`,`#c2410c`,`#a16207`];function Ci(e){let t=0;for(let n=0;n<e.length;n++)t=(t<<5)-t+e.charCodeAt(n),t|=0;return Math.abs(t)}function wi(e,t){if(e.length===0)return t;let n=e[0].trim().toLowerCase();return xi[n]||Si[Ci(n)%Si.length]}var Ti=240,Ei=100,Di=100,Oi=60,ki=360,Ai=120,ji=80,H=4,Mi=1e4,Ni=2e3,Pi=64,Fi=256,Ii=256,Li=512,Ri=.001,zi=8,Bi=16,Vi=`rgba(148, 163, 184, 0.42)`,Hi=`var(--bg-secondary)`,Ui=24,Wi=32;function Gi(e){return wi(e,Vi)}function Ki(e){return`source-${e}`}function qi(e){return`target-${e}`}function Ji(e){return`back-source-${e}`}function Yi(e){return`back-target-${e}`}function Xi(e,t){return t<=1?0:t===2?e===0?-24:Ui:(e-(t-1)/2)*Ui}function Zi(e,t=Ei){return e<=1?t:Math.max(t,(e-1)*Ui+Wi*2)}var Qi=40,$i=9,ea=18,ta=22;function na(e){let t=typeof e==`string`?e:JSON.stringify(e)??``;return Math.max(1,Math.ceil(t.length/ta))}function ra(e){let t=Qi;for(let n of Object.values(e.properties??{})){if(n==null)continue;let e=Array.isArray(n)?n:[n];for(let n of e)t+=$i+na(n)*ea}return Math.max(Ei,t)}var ia=new Set([`graph.math`,`graph.js`]),aa=[`Dictionary`,`Provider`,`Module`,`Entity`],oa={ROOT_TREE:0,DEFAULT_TREE:1,END_TREE:2};function sa(e){return e.alias.toLowerCase()===`root`||e.types.includes(`Root`)||e.types.includes(`entry_point`)}function ca(e){return e.alias.toLowerCase()===`end`||e.types.includes(`End`)}function la(e){return e.hasRoot?oa.ROOT_TREE:e.hasEnd?oa.END_TREE:oa.DEFAULT_TREE}function ua(e,t){let n=la(e)-la(t);return n===0?e.sortKey.localeCompare(t.sortKey):n}function da(e){return`real:${e}`}function fa(e){return new Map([...e].map(([e,t])=>[e,t.slice()]))}function pa(e){return new Map(e.map((e,t)=>[e.id,t]))}function ma(e,t,n){let r=(t.get(e)??[]).map(e=>n.get(e)).filter(e=>e!==void 0);if(r.length!==0)return r.reduce((e,t)=>e+t,0)/r.length}function ha(e,t,n){let r=pa(e);return e.slice().sort((e,i)=>{let a=ma(e.id,t,n),o=ma(i.id,t,n);if(a!==void 0&&o!==void 0){let e=a-o;if(Math.abs(e)>2**-52)return e}let s=r.get(e.id)-r.get(i.id);return s===0?e.stableKey.localeCompare(i.stableKey):s})}function ga(e,t){let n=Array(t+1).fill(0),r=e=>{let t=0;for(let r=e;r>0;r-=r&-r)t+=n[r];return t},i=e=>{for(let t=e;t<n.length;t+=t&-t)n[t]+=1},a=0;for(let t=e.length-1;t>=0;t--){let n=e[t]+1;a+=r(n-1),i(n)}return a}function _a(e,t){let n=new Map([...e].map(([e,t])=>[e,pa(t)])),r=new Map;for(let e of t)r.has(e.level)||r.set(e.level,[]),r.get(e.level).push(e);let i=0;for(let[e,t]of r){let r=n.get(e),a=n.get(e+1);if(!r||!a)continue;let o=t.map(e=>({sourceId:e.sourceId,targetId:e.targetId,sourcePosition:r.get(e.sourceId),targetPosition:a.get(e.targetId)})).sort((e,t)=>e.sourcePosition-t.sourcePosition||e.targetPosition-t.targetPosition||e.sourceId.localeCompare(t.sourceId)||e.targetId.localeCompare(t.targetId)).map(e=>e.targetPosition);i+=ga(o,a.size)}return i}function va(e,t,n){e.has(t)||e.set(t,[]),e.get(t).push(n)}function ya(e){return[e.source,e.target,...e.relations.map(e=>e.type)].join(`	`)}function ba(e,t){return e.nodeIntrusions<t.nodeIntrusions||e.nodeIntrusions===t.nodeIntrusions&&e.edgeCrossings<t.edgeCrossings}function xa(e){let t=new Map;for(let[n,r]of e){let e=-(r.reduce((e,t)=>e+t.height,0)+Math.max(0,r.length-1)*Oi)/2;for(let i of r)i.alias&&t.set(i.alias,{alias:i.alias,level:n,x:n*360,y:e,height:i.height}),e+=i.height+Oi}return t}function Sa(e,t,n,r,i){let a=1-i;return a*a*a*e+3*a*a*i*t+3*a*i*i*n+i*i*i*r}function Ca(e,t,n,r){let i=0,a=1;for(let o=0;o<32;o++){let o=(i+a)/2;Sa(e,t,t,n,o)<r?i=o:a=o}return(i+a)/2}function wa(e,t,n,r,i){let a=e.x+Ti,o=t.x,s=i.x+Ri,c=i.x+Ti-Ri;if(s>=o||c<=a)return null;let l=a+(o-a)/2,u=Ca(a,l,o,s),d=Ca(a,l,o,c),f=e.y+e.height/2+n,p=t.y+t.height/2+r,m=Sa(f,f,p,p,u),h=Sa(f,f,p,p,d);return{yLow:Math.min(m,h),yHigh:Math.max(m,h)}}function Ta(e,t,n,r,i){let a=wa(e,t,n,r,i);if(!a)return!1;let o=i.y+Ri,s=i.y+i.height-Ri;return a.yHigh>o&&a.yLow<s}function Ea(e,t){let n=new Map;for(let[e,r]of t)n.has(r)||n.set(r,[]),n.get(r).push(e);for(let e of n.values())e.sort();let r=[];for(let[i,a]of e.entries()){let e=t.get(a.source),o=t.get(a.target);if(!(e===void 0||o===void 0||o-e<=1)){for(let t=e+1;t<o;t++)for(let e of n.get(t)??[])if(r.push({connectionIndex:i,nodeAlias:e}),r.length>Ni)return null}}return r}function Da(e,t,n){let r=new Map,i=new Map;for(let e of t.keys())r.set(e,[]),i.set(e,[]);for(let[n,a]of e.entries()){let e=t.get(a.source),o=t.get(a.target);if(e===void 0||o===void 0)continue;let s=e>=o,c=ya(a);s?(i.get(a.source).push({connectionIndex:n,peerAlias:a.target,isBack:s,stableKey:c}),r.get(a.target).push({connectionIndex:n,peerAlias:a.source,isBack:s,stableKey:c})):(r.get(a.source).push({connectionIndex:n,peerAlias:a.target,isBack:s,stableKey:c}),i.get(a.target).push({connectionIndex:n,peerAlias:a.source,isBack:s,stableKey:c}))}let a=(e,t)=>n(e.peerAlias)-n(t.peerAlias)||e.peerAlias.localeCompare(t.peerAlias)||e.stableKey.localeCompare(t.stableKey)||e.connectionIndex-t.connectionIndex;for(let e of r.values())e.sort(a);for(let e of i.values())e.sort(a);let o=new Map,s=new Map;for(let e of r.values())for(let[t,n]of e.entries()){let r=Xi(t,e.length);n.isBack?s.set(n.connectionIndex,r):o.set(n.connectionIndex,r)}for(let e of i.values())for(let[t,n]of e.entries()){let r=Xi(t,e.length);n.isBack?o.set(n.connectionIndex,r):s.set(n.connectionIndex,r)}return{sourceOffsets:o,targetOffsets:s}}function Oa(e,t,n,r){let i=xa(e),{sourceOffsets:a,targetOffsets:o}=Da(n,t,e=>i.get(e)?.y??0),s=0;for(let{connectionIndex:e,nodeAlias:t}of r){let r=n[e],c=i.get(r.source),l=i.get(r.target),u=i.get(t);!c||!l||!u||Ta(c,l,a.get(e)??0,o.get(e)??0,u)&&(s+=1)}return s}function ka(e,t,n,r){let i=e=>t.map(t=>e.get(t).map(e=>e.id).join(`	`)).join(`
`),a=fa(e),o=1,s=a,c=n(a),l=[{candidate:a,depth:0}],u=new Set([i(a)]),d=0;for(;d<l.length&&o<r;){let{candidate:e,depth:a}=l[d++];if(!(a>=2))for(let d of t){let t=e.get(d);for(let f=0;f<t.length;f++){for(let p=0;p<t.length;p++){if(f===p)continue;let t=fa(e),m=t.get(d),[h]=m.splice(f,1);m.splice(p,0,h);let g=i(t);if(u.has(g))continue;u.add(g);let _=n(t);if(o+=1,ba(_,c)&&(s=t,c=_),a+1<2&&l.push({candidate:t,depth:a+1}),o>=r)break}if(o>=r)break}if(o>=r)break}}for(let n of t)e.set(n,s.get(n).slice());return o}function Aa(e,t,n,r){let i=new Map;for(let[t,n]of e)i.has(n)||i.set(n,[]),i.get(n).push({id:da(t),alias:t,height:r.get(t)??Ei,stableKey:`real:${t}`});for(let e of i.values())e.sort((e,t)=>e.stableKey.localeCompare(t.stableKey));let a=fa(i),o=fa(i),s=t.map((t,n)=>({connectionIndex:n,source:t.source,target:t.target,sourceLevel:e.get(t.source),targetLevel:e.get(t.target),stableKey:ya(t)})).filter(e=>e.sourceLevel!==void 0&&e.targetLevel!==void 0&&!n.has(`${e.source}\t${e.target}`)&&e.sourceLevel<e.targetLevel).sort((e,t)=>e.source.localeCompare(t.source)||e.target.localeCompare(t.target)||e.stableKey.localeCompare(t.stableKey)||e.connectionIndex-t.connectionIndex),c=s.reduce((e,t)=>e+(t.targetLevel-t.sourceLevel),0)<=Mi,l=new Map,u=new Map,d=[];for(let[e,t]of s.entries()){let n=t.sourceLevel,r=t.targetLevel;if(!c&&r-n>1)continue;let i=da(t.source),a=n;for(let s=n+1;s<r;s++){let n=`dummy:${t.source}\t${t.target}\t${e}\t${s}`;o.has(s)||o.set(s,[]),o.get(s).push({id:n,height:Ei,stableKey:n}),d.push({sourceId:i,targetId:n,level:a}),va(u,i,n),va(l,n,i),i=n,a=s}let s=da(t.target);d.push({sourceId:i,targetId:s,level:a}),va(u,i,s),va(l,s,i)}for(let e of o.values())e.sort((e,t)=>e.stableKey.localeCompare(t.stableKey));let f=[...o.keys()].sort((e,t)=>e-t);if(f.length<=1||d.length===0)return a;let p=fa(o),m=fa(o),h=Ea(t,e),g=h?.length??2001,_=h!==null,v=n=>({nodeIntrusions:_?Oa(n,e,t,h??[]):0,edgeCrossings:_a(n,d)}),y=v(m),b=()=>{let e=v(p);ba(e,y)&&(y=e,m=fa(p))},x=[...o.values()].reduce((e,t)=>e+t.length,0)<=Pi&&d.length<=Fi&&g<=Ii,S=0,C=()=>{let e=Li-S;!x||e<=1||(S+=ka(p,f,v,e),b())};for(let e=0;e<H;e++){for(let e=1;e<f.length;e++){let t=f[e],n=f[e-1];p.set(t,ha(p.get(t),l,pa(p.get(n))))}b(),C();for(let e=f.length-2;e>=0;e--){let t=f[e],n=f[e+1];p.set(t,ha(p.get(t),u,pa(p.get(n))))}b(),C()}return _&&Oa(a,e,t,h??[])<y.nodeIntrusions?a:m}function ja(e,t,n,r,i){let a=n.slice().sort((e,n)=>ya(t[e.connectionIndex]).localeCompare(ya(t[n.connectionIndex]))||e.nodeAlias.localeCompare(n.nodeAlias)||e.connectionIndex-n.connectionIndex);for(let{connectionIndex:n,nodeAlias:o}of a){let a=t[n],s=e.get(a.source),c=e.get(a.target),l=e.get(o);if(!s||!c||!l)continue;let u=wa(s,c,r.get(n)??0,i.get(n)??0,l);if(!u)continue;let d=l.y+Ri,f=l.y+l.height-Ri;if(u.yHigh<=d||u.yLow>=f)continue;let p=u.yHigh+Bi-l.y,m=l.y+l.height-(u.yLow-Bi);return p<=m?{x:l.x,y:l.y,delta:p,direction:1}:{x:l.x,y:l.y,delta:m,direction:-1}}return null}function Ma(e,t,n,r){let i=Ea(n,t);if(!(i===null||i.length===0))for(let a=0;a<zi;a++){let a=new Map;for(let[n,i]of e){let e=t.get(n);e!==void 0&&a.set(n,{alias:n,level:e,x:i.x,y:i.y,height:r.get(n)??Ei})}let{sourceOffsets:o,targetOffsets:s}=Da(n,t,e=>a.get(e)?.y??0),c=ja(a,n,i,o,s);if(!c)return;for(let[t,n]of e)n.x===c.x&&(c.direction===1?n.y>=c.y:n.y<=c.y)&&e.set(t,{x:n.x,y:n.y+c.delta*c.direction})}}function Na(e,t){if(t.has(e.alias))return`flow`;let n=e.types[0]??``,r=typeof e.properties.skill==`string`?e.properties.skill:void 0;return n===`Dictionary`?`Dictionary`:n===`Provider`?`Provider`:r&&ia.has(r)?`Module`:r?`__unknown__`:`Entity`}function Pa(e,t,n){let r=new Set;for(let e of t??[])r.add(e.source),r.add(e.target);let i=[],a=[],o=new Map;for(let t of e){let e=Na(t,r);o.set(t.alias,e),e===`flow`?i.push(t):a.push(t)}let s=new Set(i.map(e=>e.alias)),c=new Map(i.map(e=>[e.alias,e])),l=new Map,u=new Map,d=new Map;for(let e of i)l.set(e.alias,[]),u.set(e.alias,new Set),d.set(e.alias,0);for(let e of t??[])!s.has(e.source)||!s.has(e.target)||(l.get(e.source)?.push(e.target),u.get(e.source)?.add(e.target),u.get(e.target)?.add(e.source),d.set(e.target,(d.get(e.target)??0)+1));for(let e of l.values())e.sort();let f=i.filter(e=>d.get(e.alias)===0||e.types.includes(`entry_point`)||sa(e)).map(e=>e.alias).sort(),p=new Set;{let e=new Map;for(let t of i)e.set(t.alias,0);function t(t){if(e.get(t)!==0)return;e.set(t,1);let n=[{node:t,childIdx:0}];for(;n.length>0;){let t=n[n.length-1],r=l.get(t.node)??[];if(t.childIdx>=r.length){e.set(t.node,2),n.pop();continue}let i=r[t.childIdx++],a=e.get(i);a===1?p.add(`${t.node}\t${i}`):a===0&&(e.set(i,1),n.push({node:i,childIdx:0}))}}for(let e of f)t(e);for(let e of[...s].sort())t(e)}let m=[],h=new Set;for(let e of Array.from(s).sort()){if(h.has(e))continue;let t=[],n=[e];for(h.add(e);n.length>0;){let e=n.pop();t.push(e);for(let t of u.get(e)??[])h.has(t)||(h.add(t),n.push(t))}t.sort();let r=t.map(e=>c.get(e)).filter(e=>!!e);m.push({aliases:t,nodes:r,hasRoot:r.some(sa),hasEnd:r.some(ca),sortKey:t[0]??``})}m.sort(ua);let g=new Map;m.forEach((e,t)=>{e.aliases.forEach(e=>g.set(e,t))});let _=m.map(()=>[]);for(let e of t){let t=g.get(e.source),n=g.get(e.target);t!==void 0&&t===n&&_[t].push(e)}let v=new Map,y=new Map,b=0,x=0;for(let[e,t]of m.entries()){let r=new Set(t.aliases),i=t.nodes.filter(e=>d.get(e.alias)===0||e.types.includes(`entry_point`)||sa(e)).map(e=>e.alias).sort();i.length===0&&t.aliases.length>0&&i.push(t.aliases[0]);let a=new Map,o=[...i];i.forEach(e=>a.set(e,0));let s=0;for(;s<o.length;){let e=o[s++],t=a.get(e)??0;for(let n of l.get(e)??[])r.has(n)&&(p.has(`${e}\t${n}`)||(!a.has(n)||a.get(n)<=t)&&(a.set(n,t+1),o.push(n)))}let c=a.size>0?Math.max(...a.values()):0;for(let e of t.aliases)a.has(e)||a.set(e,c+1);let u=Aa(a,_[e],p,n),f=x;for(let[e,t]of[...u].sort(([e],[t])=>e-t)){let n=-(t.reduce((e,t)=>e+t.height,0)+Math.max(0,t.length-1)*Oi)/2,r=b+e,i=x+e*360;f=Math.max(f,i);for(let e of t)e.alias&&(v.set(e.alias,r),y.set(e.alias,{x:i,y:n})),n+=e.height+Oi}let m=a.size>0?Math.max(...a.values()):0;b+=m+1,x=f+Ti+ki}Ma(y,v,t,n);let S=0;for(let[e,t]of y)S=Math.max(S,t.y+(n.get(e)??Ei));let C=S+(y.size>0?Ai:0),w=new Map;for(let e of aa)w.set(e,[]);w.set(`__unknown__`,[]);for(let e of a){let t=o.get(e.alias);w.get(t).push(e.alias)}for(let e of[...aa,`__unknown__`]){let t=(w.get(e)??[]).slice().sort();if(t.length===0)continue;let r=t.reduce((e,t)=>Math.max(e,n.get(t)??Ei),0);t.forEach((e,t)=>{y.set(e,{x:0+t*360,y:C})}),C+=r+ji}return{positions:y,levelOf:v}}function Fa(e,t,n){let r=new Map,i=new Map;for(let e of t)r.set(e.source,(r.get(e.source)??0)+1),i.set(e.target,(i.get(e.target)??0)+1);return new Map(e.map(e=>{let t=Zi(Math.max(r.get(e.alias)??0,i.get(e.alias)??0),n?Di:Ei);return[e.alias,n?t:Math.max(t,ra(e))]}))}function Ia(e,t,n={}){let r=e.connections??[],i=Fa(e.nodes,r,n.compactNodes===!0);for(let[e,n]of t)i.has(e)&&Number.isFinite(n)&&n>0&&i.set(e,n);return Pa(e.nodes,r,i).positions}function La(e,t={}){let n=e.connections??[],r=t.supportsConnectionAuthoring===!0,i=t.compactNodes===!0,a=Fa(e.nodes,n,i),{positions:o,levelOf:s}=Pa(e.nodes,n,a),c=new Set;for(let[e,t]of n.entries()){let n=s.get(t.source),r=s.get(t.target);n!==void 0&&r!==void 0&&n>=r&&c.add(e)}let l=new Map,d=new Map;for(let t of e.nodes)l.set(t.alias,[]),d.set(t.alias,[]);for(let[e,t]of n.entries()){let n=ya(t);c.has(e)?(d.get(t.source).push({connIndex:e,peerAlias:t.target,isBack:!0,stableKey:n}),l.get(t.target).push({connIndex:e,peerAlias:t.source,isBack:!0,stableKey:n})):(l.get(t.source).push({connIndex:e,peerAlias:t.target,isBack:!1,stableKey:n}),d.get(t.target).push({connIndex:e,peerAlias:t.source,isBack:!1,stableKey:n}))}let f=e=>o.get(e)?.y??0,p=(e,t)=>f(e.peerAlias)-f(t.peerAlias)||e.peerAlias.localeCompare(t.peerAlias)||e.stableKey.localeCompare(t.stableKey)||e.connIndex-t.connIndex;for(let e of l.values())e.sort(p);for(let e of d.values())e.sort(p);let m=new Map,h=new Map,g=e.nodes.map(e=>{let t=l.get(e.alias)??[],n=d.get(e.alias)??[],s=Zi(Math.max(t.length,n.length),i?Di:Ei),c=Math.max(s,a.get(e.alias)??Ei),u=[],f=[],p=0,g=0;for(let e=0;e<t.length;e++){let n=t[e],r=Xi(e,t.length);if(n.isBack){let e=Yi(g++);f.push({id:e,offset:r}),h.set(n.connIndex,e)}else{let e=Ki(p++);u.push({id:e,offset:r}),m.set(n.connIndex,e)}}let _=[],v=[],y=0,b=0;for(let e=0;e<n.length;e++){let t=n[e],r=Xi(e,n.length);if(t.isBack){let e=Ji(b++);v.push({id:e,offset:r}),m.set(t.connIndex,e)}else{let e=qi(y++);_.push({id:e,offset:r}),h.set(t.connIndex,e)}}return{id:e.alias,type:e.types[0]??`default`,className:`nokey`,position:o.get(e.alias)??{x:0,y:0},width:Ti,...i?{height:c}:{initialHeight:c},style:{...di(e.types[0]??`unknown`),minHeight:s},data:{alias:e.alias,nodeType:e.types[0]??`unknown`,properties:e.properties,sourceHandles:u,targetHandles:_,backSourceHandles:v,backTargetHandles:f,supportsConnectionAuthoring:r,compact:i,minHeight:s}}}),_=[];for(let[e,t]of n.entries()){let n=t.relations.map(e=>e.type),r=`${t.source}__${t.target}__${e}`,i=Gi(n);_.push({id:r,source:t.source,target:t.target,sourceHandle:m.get(e),targetHandle:h.get(e),label:n.join(`, `),type:`bezier`,markerEnd:{type:u.ArrowClosed,width:16,height:16,color:Vi},style:{stroke:Vi,strokeWidth:2},labelStyle:{fill:i,fontSize:10,fontWeight:700},labelBgStyle:{fill:Hi,fillOpacity:.94,stroke:`rgba(15, 23, 42, 0.16)`,strokeWidth:1},labelBgPadding:[5,2],labelBgBorderRadius:6,data:{relationTypes:n}})}return{nodes:g,edges:_}}var Ra=`application/x-minigraph-clipboard-item`;function za(e){return e.includes(Ra)}function Ba(e,t){e.effectAllowed=`copy`,e.setData(Ra,t)}function Va(e){let t=e?.getData(`application/x-minigraph-clipboard-item`)??``;return t.trim()?t:null}function Ha(e,t){return e.nodes.find(e=>e.alias===t)}function Ua(e,t){return(e.connections??[]).filter(e=>e.source!==e.target&&(e.source===t||e.target===t))}var Wa={viewBox:`0 0 16 16`,width:16,height:16,fill:`none`,stroke:`currentColor`,strokeWidth:1.5,strokeLinecap:`round`,strokeLinejoin:`round`,focusable:`false`};function Ga(e){return(0,z.jsxs)(`svg`,{...Wa,...e,children:[(0,z.jsx)(`path`,{d:`M8 1.75 13.25 4.75v6.5L8 14.25l-5.25-3v-6.5L8 1.75Z`}),(0,z.jsx)(`path`,{d:`m2.95 4.9 5.05 2.9 5.05-2.9M8 7.8v6.15`})]})}function Ka(e){return(0,z.jsx)(`svg`,{...Wa,...e,children:(0,z.jsx)(`path`,{d:`m5.25 3 6.5 5-6.5 5V3Z`})})}function qa(e){return(0,z.jsxs)(`svg`,{...Wa,...e,children:[(0,z.jsx)(`rect`,{x:`5`,y:`2`,width:`8.5`,height:`9`,rx:`1.25`}),(0,z.jsx)(`path`,{d:`M10.75 14h-7A1.25 1.25 0 0 1 2.5 12.75V6A1.25 1.25 0 0 1 3.75 4.75H5`})]})}function Ja(e){return(0,z.jsxs)(`svg`,{...Wa,...e,children:[(0,z.jsx)(`path`,{d:`M8 10.5V2.75M4.75 6 8 2.75 11.25 6`}),(0,z.jsx)(`path`,{d:`M2.75 10.75v1.5a1 1 0 0 0 1 1h8.5a1 1 0 0 0 1-1v-1.5`})]})}function Ya(e){return(0,z.jsxs)(`svg`,{...Wa,...e,children:[(0,z.jsx)(`path`,{d:`M8 2.75v7.75M4.75 7.25 8 10.5l3.25-3.25`}),(0,z.jsx)(`path`,{d:`M2.75 10.75v1.5a1 1 0 0 0 1 1h8.5a1 1 0 0 0 1-1v-1.5`})]})}function Xa(e){return(0,z.jsxs)(`svg`,{...Wa,...e,children:[(0,z.jsx)(`path`,{d:`M2.75 8h7M7 5.25 9.75 8 7 10.75`}),(0,z.jsx)(`path`,{d:`M8.75 2.75h3.5a1 1 0 0 1 1 1v8.5a1 1 0 0 1-1 1h-3.5`})]})}var Za=/^[A-Za-z0-9_-]+$/,Qa=`letters, digits, hyphen or underscore`;function $a(e){return Za.test(e)}function eo(e){return(e??``).trim().replace(/[^A-Za-z0-9_-]+/g,`-`).replace(/^-+|-+$/g,``)||`untitled`}function to(e){return`${e}.json`}var no=new Set([`nodes`,`connections`]);function ro(e){return typeof e==`object`&&!!e&&!Array.isArray(e)}function io(e){if(!ro(e))return{ok:!1,error:`The file is not a JSON object with a 'nodes' section.`};let t=Object.keys(e).filter(e=>!no.has(e)).sort();if(t.length>0)return{ok:!1,error:`Unexpected top-level section${t.length>1?`s`:``}: ${t.join(`, `)}. A graph model has only 'nodes' and 'connections'.`};let n=e.nodes;if(!Array.isArray(n))return{ok:!1,error:`The 'nodes' section is mandatory and must be a list.`};if(n.length===0)return{ok:!1,error:`The 'nodes' section is empty - there is nothing to import.`};let r=null;for(let[e,t]of n.entries()){let n=`Node entry ${e+1}`;if(!ro(t))return{ok:!1,error:`${n} is not an object.`};if(typeof t.alias!=`string`||t.alias.trim()===``)return{ok:!1,error:`${n} has no alias.`};let i=t.types;if(!Array.isArray(i)||i.length===0||!i.every(e=>typeof e==`string`))return{ok:!1,error:`${n} ('${t.alias}') has no types.`};if(t.properties!==void 0&&!ro(t.properties))return{ok:!1,error:`${n} ('${t.alias}') has properties that are not an object.`};i.includes(`Root`)&&ro(t.properties)&&typeof t.properties.name==`string`&&(r=t.properties.name)}if(e.connections!==void 0&&!Array.isArray(e.connections))return{ok:!1,error:`The 'connections' section must be a list.`};let i=Array.isArray(e.connections)?e.connections:[];for(let[e,t]of i.entries())if(!ro(t)||typeof t.source!=`string`||typeof t.target!=`string`)return{ok:!1,error:`Connection entry ${e+1} needs a 'source' and a 'target'.`};return{ok:!0,model:e,name:r,nodeCount:n.length,connectionCount:i.length}}function ao(e,t){let n=JSON.parse(JSON.stringify(e)),r=n.nodes.find(e=>Array.isArray(e.types)&&e.types.includes(`Root`));return r&&(r.properties={...r.properties??{},name:t}),`${JSON.stringify(n,null,2)}\n`}function oo(){return typeof window<`u`&&typeof window.showSaveFilePicker==`function`}var so={description:`MiniGraph model (JSON)`,mimeType:`application/json`,extension:`.json`},co={description:`MiniGraph graph set`,mimeType:`application/octet-stream`,extension:`.pack`},lo={description:`OpenAPI document (YAML)`,mimeType:`application/yaml`,extension:`.yaml`};function U(e,t){return po(e,t,so)}function uo(e,t){return po(e,t,lo)}function fo(e,t){return po(e,t,co)}async function po(e,t,n){let r=window.showSaveFilePicker;if(typeof r==`function`){let i;try{i=await r.call(window,{suggestedName:t,excludeAcceptAllOption:!0,types:[{description:n.description,accept:{[n.mimeType]:[n.extension]}}]})}catch(e){if(e.name===`AbortError`)return{saved:!1,cancelled:!0};throw e}let a=await i.createWritable();return await a.write(e),await a.close(),{saved:!0,fileName:i.name}}let i=new Blob([e],{type:n.mimeType}),a=URL.createObjectURL(i),o=document.createElement(`a`);return o.href=a,o.download=t,o.rel=`noopener`,document.body.appendChild(o),o.click(),o.remove(),setTimeout(()=>URL.revokeObjectURL(a),0),{saved:!0,fileName:t}}var mo={toolbar:`_toolbar_stok4_2`,nameGroup:`_nameGroup_stok4_17`,graphName:`_graphName_stok4_24`,stats:`_stats_stok4_33`,toolbarActions:`_toolbarActions_stok4_53`,runControls:`_runControls_stok4_59`,tooltipAnchor:`_tooltipAnchor_stok4_65`,actionTooltip:`_actionTooltip_stok4_76`,toolbarButton:`_toolbarButton_stok4_116`,toolbarIconButton:`_toolbarIconButton_stok4_134`,toolbarIcon:`_toolbarIcon_stok4_134`};function ho({graphData:e,graphName:t,onCopySuccess:n,onCopyError:r,extraActions:i,onImport:a,importDisabledReason:o=null,onDownload:s}){let c=(0,P.useCallback)(()=>{e&&navigator.clipboard.writeText(JSON.stringify(e,null,2)).then(()=>n?.()).catch(()=>r?.())},[e,n,r]),l=e?.nodes.length??0,u=(e?.connections??[]).length;return(0,z.jsxs)(`div`,{className:mo.toolbar,children:[(0,z.jsxs)(`div`,{className:mo.nameGroup,children:[(0,z.jsx)(`span`,{className:mo.graphName,children:t??`Untitled`}),(0,z.jsxs)(`span`,{className:mo.stats,children:[l,` node`,l===1?``:`s`,` · `,u,` connection`,u===1?``:`s`]})]}),(0,z.jsxs)(`div`,{className:mo.toolbarActions,children:[i,a&&(0,z.jsx)(`button`,{type:`button`,className:`${mo.toolbarButton} ${mo.toolbarIconButton}`,onClick:a,disabled:o!==null,title:o??`Import a graph model from a JSON file (replaces the current draft)`,"aria-label":`Import graph model from a JSON file`,children:(0,z.jsx)(Xa,{className:mo.toolbarIcon,"aria-hidden":`true`,focusable:`false`})}),s&&(0,z.jsx)(`button`,{type:`button`,className:`${mo.toolbarButton} ${mo.toolbarIconButton}`,onClick:s,disabled:!e,title:e?`Download the graph as ${to(eo(t))}`:`No graph to download`,"aria-label":`Download graph as a JSON file`,children:(0,z.jsx)(Ya,{className:mo.toolbarIcon,"aria-hidden":`true`,focusable:`false`})}),(0,z.jsx)(`button`,{type:`button`,className:`${mo.toolbarButton} ${mo.toolbarIconButton}`,onClick:c,title:`Copy raw graph JSON to clipboard`,"aria-label":`Copy raw graph JSON to clipboard`,children:(0,z.jsx)(qa,{className:mo.toolbarIcon,"aria-hidden":`true`,focusable:`false`})})]})]})}var go=new Set([`instantiating`,`outcome-uncertain`]);function _o(e){switch(e){case`instantiating`:return`Instantiating…`;case`outcome-uncertain`:return`Waiting…`;default:return`Instantiate`}}function vo(e){switch(e){case`instantiating`:return`Graph is being instantiated`;case`outcome-uncertain`:return`Waiting for the backend graph outcome`;default:return`Instantiate graph`}}function yo(e){switch(e){case`instantiating`:return`Graph is being instantiated`;case`running`:return`Graph is running`;case`outcome-uncertain`:return`Waiting for the backend graph outcome`;default:return``}}function bo(e,t){return t?`${e} ${/[.!?]$/.test(t)?t:`${t}.`}`:e}function xo(e,t){let n=[];return e.length>0&&n.push(`${e.length} input.body ${e.length===1?`path`:`paths`}`),t.length>0&&n.push(`${t.length} input ${t.length===1?`header`:`headers`}`),n.length===0?`This graph reads neither input.body nor input.header, so uploading is optional`:`This graph reads ${n.join(` and `)}`}function So({id:e,text:t,keyboardFallback:n,children:r}){let[i,a]=(0,P.useState)(!1),[o,s]=(0,P.useState)(!1);return(0,z.jsxs)(`span`,{className:mo.tooltipAnchor,onMouseEnter:()=>a(!0),onMouseLeave:()=>a(!1),onFocusCapture:()=>s(!0),onBlurCapture:e=>{e.currentTarget.contains(e.relatedTarget)||s(!1)},tabIndex:n?0:void 0,"aria-describedby":n?e:void 0,children:[r,(0,z.jsx)(`span`,{id:e,className:mo.actionTooltip,role:`tooltip`,"data-state":i||o?`open`:`closed`,children:t})]})}function Co({phase:e,canInstantiate:t,canUpload:n,canRun:r,disabledReason:i,inputBodyPaths:a=[],inputHeaderNames:o=[],onInstantiate:s,onUpload:c,onRun:l}){let u=(0,P.useId)(),d=`${u}-instantiate`,f=`${u}-upload`,p=`${u}-run`,m=go.has(e),h=yo(e),g=i||h||(e===`ready`?`Instantiating again starts from a fresh instance`:``),_=i||h||(n?xo(a,o):`Instantiate the graph first`),v=i||h||(r?``:`Instantiate the graph first`),y=bo(`Create a runnable instance of the current graph.`,g),b=bo(`Upload a JSON payload as the mock input.body of the instance, and optional mock headers as its input.header. Only you see the form.`,_),x=bo(`Run the instantiated graph, with the uploaded mock input if any.`,v);return(0,z.jsxs)(`div`,{className:mo.runControls,role:`group`,"aria-label":`Graph run controls`,children:[(0,z.jsx)(So,{id:d,text:y,keyboardFallback:!t,children:(0,z.jsxs)(`button`,{type:`button`,className:mo.toolbarButton,onClick:s,disabled:!t,"aria-label":vo(e),"aria-describedby":d,"aria-busy":m,children:[(0,z.jsx)(Ga,{className:mo.toolbarIcon,"aria-hidden":`true`,focusable:`false`}),(0,z.jsx)(`span`,{children:_o(e)})]})}),(0,z.jsx)(So,{id:f,text:b,keyboardFallback:!n,children:(0,z.jsxs)(`button`,{type:`button`,className:mo.toolbarButton,onClick:c,disabled:!n,"aria-label":`Upload mock input`,"aria-describedby":f,children:[(0,z.jsx)(Ja,{className:mo.toolbarIcon,"aria-hidden":`true`,focusable:`false`}),(0,z.jsx)(`span`,{children:`Upload`})]})}),(0,z.jsx)(So,{id:p,text:x,keyboardFallback:!r,children:(0,z.jsxs)(`button`,{type:`button`,className:mo.toolbarButton,onClick:l,disabled:!r,"aria-label":e===`ready`?`Run instantiated graph`:e===`running`?`Graph is running`:`Run graph`,"aria-describedby":p,"aria-busy":e===`running`,children:[(0,z.jsx)(Ka,{className:mo.toolbarIcon,"aria-hidden":`true`,focusable:`false`}),(0,z.jsx)(`span`,{children:e===`running`?`Running…`:`Run`})]})})]})}var wo={menu:`_menu_13qxg_1`,menuItem:`_menuItem_13qxg_12`};function To({open:e,x:t,y:n,canCreateNode:r,onCreateNode:i,onClose:a}){let o=(0,P.useRef)(null),s=(0,P.useRef)(null);return(0,P.useEffect)(()=>{if(!e)return;s.current?.focus();let t=e=>{o.current&&!o.current.contains(e.target)&&a()},n=e=>{e.key===`Escape`&&(e.preventDefault(),a())};return document.addEventListener(`pointerdown`,t),document.addEventListener(`keydown`,n),()=>{document.removeEventListener(`pointerdown`,t),document.removeEventListener(`keydown`,n)}},[e,a]),e?(0,z.jsx)(`div`,{ref:o,className:wo.menu,style:{left:t,top:n},role:`menu`,"aria-label":`Graph actions`,children:(0,z.jsx)(`button`,{ref:s,role:`menuitem`,type:`button`,className:wo.menuItem,disabled:!r,onClick:()=>{r&&(i(),a())},children:`Create Node`})}):null}var Eo={menu:`_menu_1trgd_1`,menuItem:`_menuItem_1trgd_12`,dangerItem:`_dangerItem_1trgd_38`,confirmation:`_confirmation_1trgd_51`,confirmationText:`_confirmationText_1trgd_57`,confirmationActions:`_confirmationActions_1trgd_65`},Do=8;function Oo(e){let{open:t,x:n,y:r,onClose:i}=e,[a,o]=(0,P.useState)(!1),[s,c]=(0,P.useState)({left:n,top:r}),l=(0,P.useRef)(null),u=(0,P.useRef)(null),d=(0,P.useRef)(null),f=e.mode===`multi-node`?e.selectedCount:null,p=f!==null&&f>1,m=e.mode===`multi-node`?p&&e.canClipSelectedNodes:e.canClipNode,h=e.mode===`single-node`&&e.canConnectNode,g=e.mode===`single-node`&&e.canEditNode,_=e.mode===`multi-node`?p&&e.canDeleteSelectedNodes:e.canDeleteNode,v=m||h||g||_,y=p?`${f} selected nodes`:e.mode===`single-node`?e.nodeAlias:``;return(0,P.useLayoutEffect)(()=>{t&&o(!1)},[t,y,n,r]),(0,P.useLayoutEffect)(()=>{if(!t)return;let e=l.current;if(!e){c({left:n,top:r});return}let i=e.getBoundingClientRect(),a=Math.max(Do,window.innerWidth-i.width-Do),o=Math.max(Do,window.innerHeight-i.height-Do);c({left:Math.min(Math.max(n,Do),a),top:Math.min(Math.max(r,Do),o)})},[m,_,g,a,t,y,n,r]),(0,P.useEffect)(()=>{if(!t){o(!1);return}a?d.current?.focus():u.current?.focus()},[a,t]),(0,P.useEffect)(()=>{if(!t)return;let e=e=>{l.current&&!l.current.contains(e.target)&&i()},n=e=>{e.key===`Escape`&&(e.preventDefault(),i())},r=()=>i();return document.addEventListener(`pointerdown`,e),document.addEventListener(`keydown`,n),window.addEventListener(`scroll`,r,!0),window.addEventListener(`resize`,r),()=>{document.removeEventListener(`pointerdown`,e),document.removeEventListener(`keydown`,n),window.removeEventListener(`scroll`,r,!0),window.removeEventListener(`resize`,r)}},[i,t]),!t||!v?null:(0,z.jsx)(`div`,{ref:l,className:Eo.menu,style:{left:s.left,top:s.top},role:`menu`,"aria-label":p?`Actions for ${f} selected nodes`:`Node actions for ${y}`,children:a?(0,z.jsxs)(`div`,{className:Eo.confirmation,role:`group`,"aria-label":`Confirm delete ${y}`,children:[(0,z.jsx)(`div`,{className:Eo.confirmationText,children:p?`Delete ${f} selected nodes?`:`Delete "${y}"?`}),(0,z.jsxs)(`div`,{className:Eo.confirmationActions,children:[(0,z.jsx)(`button`,{ref:d,type:`button`,className:`${Eo.menuItem} ${Eo.dangerItem}`,onClick:()=>{e.mode===`multi-node`?e.onDeleteSelectedNodes():e.onDeleteNode(),i()},children:`Delete`}),(0,z.jsx)(`button`,{type:`button`,className:Eo.menuItem,onClick:()=>o(!1),children:`Cancel`})]})]}):(0,z.jsxs)(z.Fragment,{children:[m&&(0,z.jsx)(`button`,{ref:u,role:`menuitem`,type:`button`,className:Eo.menuItem,onClick:()=>{e.mode===`multi-node`?e.onClipSelectedNodes():e.onClipNode(),i()},children:p?`Clip ${f} selected nodes to Workspace`:`Clip to Workspace`}),h&&e.mode===`single-node`&&(0,z.jsx)(`button`,{ref:m?void 0:u,role:`menuitem`,type:`button`,className:Eo.menuItem,onClick:()=>{e.onConnectNode(),i()},children:`Connect to…`}),g&&e.mode===`single-node`&&(0,z.jsx)(`button`,{ref:m||h?void 0:u,role:`menuitem`,type:`button`,className:Eo.menuItem,onClick:()=>{e.onEditNode(),i()},children:`Edit Node`}),_&&(0,z.jsx)(`button`,{ref:!m&&!h&&!g?u:void 0,role:`menuitem`,type:`button`,className:`${Eo.menuItem} ${Eo.dangerItem}`,onClick:()=>o(!0),children:p?`Delete ${f} selected nodes`:`Delete Node`})]})})}var ko=8;function Ao({open:e,x:t,y:n,sourceAlias:r,targetAlias:i,relations:a,onDeleteRelation:o,onClose:s}){let[c,l]=(0,P.useState)({left:t,top:n}),u=(0,P.useRef)(null),d=(0,P.useRef)(null);return(0,P.useLayoutEffect)(()=>{if(!e)return;let r=u.current;if(!r){l({left:t,top:n});return}let i=r.getBoundingClientRect(),a=Math.max(ko,window.innerWidth-i.width-ko),o=Math.max(ko,window.innerHeight-i.height-ko);l({left:Math.min(Math.max(t,ko),a),top:Math.min(Math.max(n,ko),o)})},[e,a,t,n]),(0,P.useEffect)(()=>{e&&d.current?.focus()},[e]),(0,P.useEffect)(()=>{if(!e)return;let t=e=>{u.current&&!u.current.contains(e.target)&&s()},n=e=>{e.key===`Escape`&&(e.preventDefault(),s())},r=()=>s();return document.addEventListener(`pointerdown`,t),document.addEventListener(`keydown`,n),window.addEventListener(`scroll`,r,!0),window.addEventListener(`resize`,r),()=>{document.removeEventListener(`pointerdown`,t),document.removeEventListener(`keydown`,n),window.removeEventListener(`scroll`,r,!0),window.removeEventListener(`resize`,r)}},[s,e]),!e||a.length===0?null:(0,z.jsxs)(`div`,{ref:u,className:Eo.menu,style:{left:c.left,top:c.top},role:`menu`,"aria-label":`Connection actions for ${r} → ${i}`,children:[a.map((e,t)=>(0,z.jsxs)(`button`,{ref:t===0?d:void 0,role:`menuitem`,type:`button`,className:`${Eo.menuItem} ${Eo.dangerItem}`,onClick:()=>{o(e),s()},children:[`Delete '`,e,`'`]},`${e}-${t}`)),a.length>1&&(0,z.jsxs)(`button`,{role:`menuitem`,type:`button`,className:`${Eo.menuItem} ${Eo.dangerItem}`,onClick:()=>{o(void 0),s()},children:[`Delete all (`,a.length,`)`]})]})}var jo={island:`_island_had6p_6`,islandDragging:`_islandDragging_had6p_18`,islandGrip:`_islandGrip_had6p_22`,islandGripDots:`_islandGripDots_had6p_46`,islandGripLabel:`_islandGripLabel_had6p_51`,minimap:`_minimap_had6p_55`,controls:`_controls_had6p_59`,toggleButton:`_toggleButton_had6p_63`,toggleIcon:`_toggleIcon_had6p_68`},Mo={Root:`#15803d`,End:`#dc2626`,Fetcher:`#2563eb`,mapper:`#ea580c`,Math:`#a16207`,JavaScript:`#7e22ce`,Provider:`#be185d`,Dictionary:`#0e7490`,Join:`#65a30d`,Extension:`#4338ca`,Island:`#475569`,Decision:`#b45309`},No={left:50,bottom:15},Po=8,Fo=`graph-minimap-position`;function Io(e){return Mo[e.type??``]??`#6c7086`}function Lo(e){return e instanceof Element&&e.closest(`input, textarea, select, [contenteditable]:not([contenteditable="false"])`)!==null}function Ro(e,t,n){if(t.clientWidth<=0||t.clientHeight<=0||n.offsetWidth<=0||n.offsetHeight<=0)return e;let r=Math.max(t.clientWidth-n.offsetWidth-Po,Po),i=Math.max(t.clientHeight-n.offsetHeight-Po,Po);return{left:Math.min(Math.max(e.left,Po),r),bottom:Math.min(Math.max(e.bottom,Po),i)}}function zo({open:e,onOpenChange:t,hotkeyEnabled:n,children:r}){let i=e?`Hide minimap`:`Show minimap`,a=(0,P.useCallback)(()=>{t(!e)},[t,e]);(0,P.useEffect)(()=>{if(!n)return;let e=e=>{e.defaultPrevented||e.repeat||!e.ctrlKey||e.metaKey||e.altKey||e.shiftKey||e.code!==`KeyM`||Lo(e.target)||(e.preventDefault(),a())};return window.addEventListener(`keydown`,e),()=>window.removeEventListener(`keydown`,e)},[n,a]);let[o,s]=ce(Fo,No),c=(0,P.useRef)(null),[l,u]=(0,P.useState)(null),d=(0,P.useCallback)(e=>{e.pointerType===`mouse`&&e.button!==0||(e.preventDefault(),u({pointerId:e.pointerId,startX:e.clientX,startY:e.clientY,origin:o}))},[o]);return(0,P.useEffect)(()=>{if(l===null)return;let e=e=>{if(e.pointerId!==l.pointerId)return;let t=c.current,n=t?.offsetParent instanceof HTMLElement?t.offsetParent:null,r={left:l.origin.left+(e.clientX-l.startX),bottom:l.origin.bottom-(e.clientY-l.startY)};s(t&&n?Ro(r,n,t):r)},t=e=>{e.pointerId===l.pointerId&&u(null)};return window.addEventListener(`pointermove`,e),window.addEventListener(`pointerup`,t),window.addEventListener(`pointercancel`,t),()=>{window.removeEventListener(`pointermove`,e),window.removeEventListener(`pointerup`,t),window.removeEventListener(`pointercancel`,t)}},[l,s]),(0,P.useEffect)(()=>{if(!e)return;let t=c.current,n=t?.offsetParent instanceof HTMLElement?t.offsetParent:null;if(!t||!n)return;let r=()=>{s(e=>{let r=Ro(e,n,t);return r.left===e.left&&r.bottom===e.bottom?e:r})};if(r(),typeof ResizeObserver>`u`)return;let i=new ResizeObserver(r);return i.observe(n),()=>i.disconnect()},[e,s]),(0,z.jsxs)(z.Fragment,{children:[e&&(0,z.jsxs)(`div`,{ref:c,className:l===null?jo.island:`${jo.island} ${jo.islandDragging}`,style:{left:o.left,bottom:o.bottom},role:`group`,"aria-label":`Graph minimap`,children:[(0,z.jsxs)(`div`,{className:jo.islandGrip,role:`button`,"aria-label":`Move minimap`,title:`Drag to move the minimap`,onPointerDown:d,children:[(0,z.jsx)(`span`,{className:jo.islandGripDots,"aria-hidden":`true`,children:`⠿`}),(0,z.jsx)(`span`,{className:jo.islandGripLabel,children:`Minimap`})]}),(0,z.jsx)(g,{className:jo.minimap,nodeColor:Io,maskColor:`rgba(0,0,0,0.3)`,pannable:!0,style:{position:`relative`,margin:0,background:`#fff`}})]}),(0,z.jsxs)(f,{position:`bottom-left`,showInteractive:!1,className:jo.controls,children:[r,(0,z.jsx)(v,{className:`${jo.toggleButton} nodrag nopan`,"aria-label":i,"aria-keyshortcuts":`Control+M`,"aria-pressed":e,title:`${i} (Ctrl + M)`,onClick:a,children:(0,z.jsxs)(`svg`,{className:jo.toggleIcon,viewBox:`1.5 2 17 16`,fill:`none`,"aria-hidden":`true`,focusable:`false`,children:[(0,z.jsx)(`rect`,{x:`2.5`,y:`3`,width:`15`,height:`14`,rx:`1.5`}),(0,z.jsx)(`path`,{d:`M6 12.5 9 8l2.5 2 2.5-3`}),(0,z.jsx)(`circle`,{cx:`6`,cy:`12.5`,r:`1`}),(0,z.jsx)(`circle`,{cx:`9`,cy:`8`,r:`1`}),(0,z.jsx)(`circle`,{cx:`11.5`,cy:`10`,r:`1`}),(0,z.jsx)(`circle`,{cx:`14`,cy:`7`,r:`1`})]})})]})]})}function Bo(e){return e.trim().toLowerCase()}function Vo(e){let t=new Set;return e.filter(e=>{let n=Bo(e);return t.has(n)?!1:(t.add(n),!0)})}function Ho(e,t){let n=Vo(t),r=Bo(e);return n.length>1&&n.some(e=>Bo(e)===r)?{kind:`multi-node`,aliases:n}:{kind:`single-node`,alias:e}}function Uo(e,t){let n=new Map(t.nodes.map(e=>[Bo(e.alias),e]));return Vo(e).map(e=>n.get(Bo(e))).filter(e=>e!==void 0)}var Wo=[],Go=[],Ko=[`Shift`,`Control`,`Meta`];function qo(e,t){return e.length===t.length&&e.every((e,n)=>e===t[n])}function Jo({graphData:e,graphName:t,onCopySuccess:n,onCopyError:r,graphRunControls:i,onRenderError:a,isRefreshing:o=!1,onClipNode:s,onClipNodes:u,onClipboardDrop:d,onImportFiles:f,onImportGraph:p,onDownload:g,isActive:_,isConnected:y,supportsAuthoring:S=!1,onCreateNode:C,onCreateConnection:w,onEditNode:ee,onDeleteNode:T,onDeleteNodes:E,onDeleteConnections:D,panelLayoutKey:O}){let[k,A]=(0,P.useState)(null),[te,ne]=(0,P.useState)(null),[re,j]=(0,P.useState)(null),[M,N]=(0,P.useState)([]),[ie,F]=(0,P.useState)(!1),[ae,I]=(0,P.useState)(!1),[L,oe]=(0,P.useState)(!1),se=(0,P.useRef)(0),le=(0,P.useRef)(0),ue=!!(S&&C&&y),R=!!(S&&w&&y),de=!!s,fe=!!u,pe=!!(S&&ee&&y),me=!!(S&&T&&y),he=!!(S&&E&&y),ge=de||pe||me||R,_e=fe||he,ve=ge||_e,ye=!!(d&&y),be=!!f,xe=(0,P.useCallback)(()=>{se.current=0,F(!1),le.current=0,I(!1)},[]);(0,P.useEffect)(()=>{if(!te)return;let e=e=>{e.key===`Escape`&&ne(null)},t=()=>ne(null);return document.addEventListener(`keydown`,e),window.addEventListener(`scroll`,t,!0),window.addEventListener(`resize`,t),()=>{document.removeEventListener(`keydown`,e),window.removeEventListener(`scroll`,t,!0),window.removeEventListener(`resize`,t)}},[te]),(0,P.useEffect)(()=>{let e=()=>xe();return window.addEventListener(`dragend`,e),window.addEventListener(`drop`,e),()=>{window.removeEventListener(`dragend`,e),window.removeEventListener(`drop`,e),xe()}},[xe]);let Se=(0,P.useRef)(a);(0,P.useEffect)(()=>{Se.current=a},[a]);let[Ce,we]=ce(`graph-nodes-compact`,!1),{nodes:Te,edges:Ee,transformError:De}=(0,P.useMemo)(()=>{if(!e)return{nodes:Wo,edges:Go,transformError:null};try{return{...La(e,{supportsConnectionAuthoring:R,compactNodes:Ce}),transformError:null}}catch(e){return{nodes:Wo,edges:Go,transformError:e instanceof Error?e.message:String(e)}}},[R,Ce,e]);(0,P.useEffect)(()=>{De&&Se.current?.(`Graph render failed: ${De}`)},[De]);let Oe=(0,P.useMemo)(()=>e?JSON.stringify(e.nodes.map(e=>e.alias)):`empty`,[e]),[ke,Ae,je]=l(Te),[Me,Ne,Pe]=x(Ee),Fe=(0,P.useRef)(null),Ie=!!(e&&e.nodes.length>0),Le=(0,P.useCallback)(({nodes:e})=>{let t=e.map(e=>e.data.alias);N(e=>qo(e,t)?e:t)},[]);(0,P.useEffect)(()=>{Ae(Te),Ne(Ee),N([]),A(null),j(null)},[Te,Ee,Ae,Ne]);let Re=(0,P.useRef)(null),ze=(0,P.useRef)(null);(0,P.useEffect)(()=>{if(!e||e.nodes.length===0)return;let t=ze.current;if(t&&t.graph===e&&t.compact===Ce)return;let n=new Map(e.nodes.map(e=>[e.alias,e.properties]));if(!(ke.length===n.size&&ke.every(e=>n.get(e.id)===e.data.properties&&e.data.compact===Ce)))return;let r=new Map;for(let e of ke){let t=e.measured?.height;if(typeof t!=`number`)return;r.set(e.id,t)}ze.current={graph:e,compact:Ce};let i=Ia(e,r,{compactNodes:Ce});ke.some(e=>{let t=i.get(e.id);return t!==void 0&&(t.x!==e.position.x||t.y!==e.position.y)})&&Ae(e=>e.map(e=>{let t=i.get(e.id);return t?{...e,position:t}:e})),requestAnimationFrame(()=>{Re.current?.fitView({padding:.1})})},[Ce,e,ke,Ae]);let Be=e=>{ye&&za(Array.from(e.dataTransfer.types))&&(e.preventDefault(),se.current+=1,F(!0))},Ve=e=>{ye&&za(Array.from(e.dataTransfer.types))&&(e.preventDefault(),e.dataTransfer.dropEffect=`copy`,F(!0))},He=e=>{za(Array.from(e.dataTransfer.types))&&(se.current=Math.max(0,se.current-1),se.current===0&&F(!1))},Ue=e=>{if(!ye||!za(Array.from(e.dataTransfer.types)))return;e.preventDefault();let t=Va(e.dataTransfer);xe(),t&&d?.(t)},We=e=>Array.from(e.types).includes(`Files`),Ge=e=>{if(!We(e.dataTransfer)){Be(e);return}be&&(e.preventDefault(),le.current+=1,I(!0))},Ke=e=>{if(!We(e.dataTransfer)){Ve(e);return}be&&(e.preventDefault(),e.dataTransfer.dropEffect=`copy`,I(!0))},qe=e=>{if(!We(e.dataTransfer)){He(e);return}le.current=Math.max(0,le.current-1),le.current===0&&I(!1)},Je=e=>{if(!We(e.dataTransfer)){Ue(e);return}if(!be)return;e.preventDefault();let t=e.dataTransfer.files;xe(),t.length>0&&f?.(t)},Ye=(0,P.useMemo)(()=>new Set(e?.nodes.map(e=>e.alias)??[]),[e]),Xe=k?.target.kind===`single-node`&&e?Ha(e,k.target.alias):null,Ze=k?.target.kind===`multi-node`?k.target.aliases:[],Qe=e?Uo(Ze,e):[],$e=(0,P.useCallback)(e=>!R||!e.source||!e.target||e.source===e.target||!Ye.has(e.source)||!Ye.has(e.target)?!1:e.sourceHandle===`authoring-source`&&e.targetHandle===`authoring-target`,[R,Ye]),et=(0,P.useRef)(null);(0,P.useEffect)(()=>{let e=e=>{et.current={x:e.clientX,y:e.clientY}};return document.addEventListener(`pointerup`,e,!0),()=>document.removeEventListener(`pointerup`,e,!0)},[]);let tt=(0,P.useCallback)(e=>{$e(e)&&(!e.source||!e.target||w?.(e.source,e.target,et.current??void 0))},[$e,w]),[nt,rt]=(0,P.useState)(null);(0,P.useEffect)(()=>{rt(null)},[e,R]),(0,P.useEffect)(()=>{if(nt===null)return;let e=e=>{e.key===`Escape`&&(e.preventDefault(),rt(null))};return document.addEventListener(`keydown`,e),()=>document.removeEventListener(`keydown`,e)},[nt]);let it=(0,P.useCallback)((e,t)=>{t.handleId!==`authoring-source`||t.handleType!==`source`||(Fe.current=t.nodeId,A(null),ne(null),j(null))},[]),at=(0,P.useCallback)(()=>{Fe.current=null},[]),ot=(0,P.useRef)(O);(0,P.useEffect)(()=>{if(ot.current===O||(ot.current=O,!Ie))return;let e=null,t=requestAnimationFrame(()=>{e=requestAnimationFrame(()=>{Re.current?.fitView({padding:.1})})});return()=>{cancelAnimationFrame(t),e!==null&&cancelAnimationFrame(e)}},[Ie,O]);let st=!!(S&&D&&y),ct=(0,P.useCallback)(async({edges:e})=>{if(st&&e.length>0){let t=new Set,n=[];for(let r of e){let e=`${r.source}\t${r.target}`;t.has(e)||(t.add(e),n.push({source:r.source,target:r.target}))}D?.(n)}return!1},[st,D]);return De?(0,z.jsxs)(`div`,{className:vi.empty,children:[(0,z.jsx)(`span`,{className:vi.emptyIcon,children:`⚠️`}),(0,z.jsx)(`span`,{children:`Graph could not be rendered.`}),(0,z.jsx)(`span`,{children:De})]}):(0,z.jsx)(yi,{onRenderError:a,children:(0,z.jsxs)(`div`,{className:vi.graphWrapper,"aria-busy":o,children:[Ie&&e&&(0,z.jsx)(ho,{graphData:e,graphName:t,onCopySuccess:n,onCopyError:r,extraActions:i?(0,z.jsx)(Co,{...i}):void 0,onImport:p,importDisabledReason:y?null:`Connect first to import a graph`,onDownload:g}),(0,z.jsxs)(`div`,{className:vi.graphSurface,"data-connect-picking":nt!==null||void 0,onDragEnter:Ge,onDragOver:Ke,onDragLeave:qe,onDrop:Je,children:[Ie?(0,z.jsxs)(c,{nodes:ke,edges:Me,onInit:e=>{Re.current=e},onNodesChange:je,onEdgesChange:Pe,nodesConnectable:R,edgesReconnectable:!1,connectOnClick:!1,isValidConnection:$e,onConnect:tt,onConnectStart:it,onConnectEnd:at,deleteKeyCode:[`Delete`,`Backspace`],onBeforeDelete:ct,nodeTypes:_i,fitView:!0,fitViewOptions:{padding:.1},minZoom:.2,maxZoom:4,zoomOnScroll:!0,zoomOnPinch:!0,zoomOnDoubleClick:!0,panOnScroll:!1,selectionKeyCode:`Shift`,multiSelectionKeyCode:Ko,selectionOnDrag:!1,selectionMode:m.Partial,proOptions:{hideAttribution:!1},onSelectionChange:Le,onNodeContextMenu:(e,t)=>{if(e.preventDefault(),e.stopPropagation(),ne(null),j(null),!ve)return;let n=Ho(t.data.alias,M);n.kind===`single-node`&&M.length>1&&(Ae(e=>e.map(e=>({...e,selected:e.data.alias===t.data.alias}))),N([t.data.alias])),A({x:e.clientX,y:e.clientY,target:n})},onEdgeContextMenu:(e,t)=>{e.preventDefault(),e.stopPropagation(),A(null),ne(null),st&&j({x:e.clientX,y:e.clientY,source:t.source,target:t.target,relations:t.data?.relationTypes??[]})},onPaneContextMenu:e=>{e.preventDefault(),A(null),j(null),ue&&ne({x:e.clientX,y:e.clientY})},onPaneClick:()=>{A(null),ne(null),j(null),rt(null)},onNodeClick:(e,t)=>{if(nt===null)return;e.preventDefault(),e.stopPropagation();let n=t.data.alias;n!==nt&&w?.(nt,n,{x:e.clientX,y:e.clientY}),rt(null)},children:[(0,z.jsx)(b,{variant:h.Dots,gap:18,size:1,color:`rgba(255,255,255,0.07)`}),(0,z.jsx)(zo,{open:L,onOpenChange:oe,hotkeyEnabled:_,children:(0,z.jsx)(v,{onClick:()=>we(e=>!e),title:Ce?`Show node details`:`Show thumbnail nodes`,"aria-label":Ce?`Show node details`:`Show thumbnail nodes`,"aria-pressed":Ce,children:(0,z.jsxs)(`span`,{className:vi.detailToggleIcon,"aria-hidden":`true`,children:[(0,z.jsx)(`span`,{className:vi.detailToggleHeader}),Ce&&(0,z.jsxs)(z.Fragment,{children:[(0,z.jsx)(`span`,{className:vi.detailToggleLine}),(0,z.jsx)(`span`,{className:vi.detailToggleLine}),(0,z.jsx)(`span`,{className:vi.detailToggleLine})]})]})})})]}):(0,z.jsxs)(`div`,{className:vi.empty,children:[(0,z.jsx)(`span`,{className:vi.emptyIcon,children:`🕸️`}),(0,z.jsx)(`span`,{children:`No graph data yet.`}),(0,z.jsxs)(`span`,{children:[`Run `,(0,z.jsx)(`strong`,{children:`create node`}),` or `,(0,z.jsx)(`strong`,{children:`import graph from`}),` in the playground`,f?`, or drop a graph JSON file here`:``,` — the view follows the live session graph.`]}),(S&&C||p)&&(0,z.jsxs)(z.Fragment,{children:[(0,z.jsxs)(`div`,{className:vi.emptyActions,children:[S&&C&&(0,z.jsx)(`button`,{type:`button`,className:vi.emptyCreateButton,disabled:!y,onClick:()=>C(`empty-graph`),children:`Create Node`}),p&&(0,z.jsx)(`button`,{type:`button`,className:vi.emptySecondaryButton,disabled:!y,onClick:p,children:`Import Graph`})]}),!y&&(0,z.jsx)(`span`,{className:vi.emptyHint,children:`Connect WebSocket to create or import a graph.`})]})]}),nt!==null&&(0,z.jsxs)(`div`,{className:vi.connectBanner,role:`status`,children:[(0,z.jsxs)(`span`,{children:[`Connecting from `,(0,z.jsx)(`strong`,{children:nt}),` — click a target node`]}),(0,z.jsx)(`button`,{type:`button`,className:vi.connectBannerCancel,onClick:()=>rt(null),children:`Cancel (Esc)`})]}),o&&(0,z.jsx)(`div`,{className:vi.refreshingOverlay,children:(0,z.jsx)(`div`,{className:vi.refreshingSpinner,role:`status`,"aria-label":`Graph refreshing`})}),ie&&(0,z.jsx)(`div`,{className:vi.clipboardDropOverlay,children:(0,z.jsx)(`div`,{className:vi.clipboardDropMessage,children:`Drop to paste workspace node`})}),ae&&(0,z.jsx)(`div`,{className:vi.fileDropOverlay,children:(0,z.jsx)(`div`,{className:vi.fileDropMessage,children:y?`Drop to import the graph file as the draft`:`Connect first to import a graph file`})}),(0,z.jsx)(To,{open:te!==null,x:te?.x??0,y:te?.y??0,canCreateNode:ue,onCreateNode:()=>C?.(`pane-context-menu`),onClose:()=>ne(null)}),k?.target.kind===`multi-node`?(0,z.jsx)(Oo,{mode:`multi-node`,open:Qe.length>1&&_e,x:k.x,y:k.y,selectedCount:Ze.length,canClipSelectedNodes:fe,canDeleteSelectedNodes:he,onClipSelectedNodes:()=>{if(!e){u?.([]);return}let t=Qe.map(t=>({node:t,connections:Ua(e,t.alias)}));u?.(t)},onDeleteSelectedNodes:()=>{let e=Qe.length===Ze.length;E?.(e?Qe:[])},onClose:()=>A(null)}):(0,z.jsx)(Oo,{mode:`single-node`,open:k!==null&&Xe!==null&&ge,x:k?.x??0,y:k?.y??0,nodeAlias:k?.target.kind===`single-node`?k.target.alias:``,canClipNode:de&&Xe!==null,canConnectNode:R&&Xe!==null,canEditNode:pe&&Xe!==null,canDeleteNode:me&&Xe!==null,onConnectNode:()=>{Xe&&rt(Xe.alias)},onClipNode:()=>{if(!Xe||!e)return;let t=Ua(e,Xe.alias);s?.(Xe,t)},onEditNode:()=>{Xe&&ee?.(Xe)},onDeleteNode:()=>{Xe&&T?.(Xe)},onClose:()=>A(null)}),(0,z.jsx)(Ao,{open:re!==null&&st,x:re?.x??0,y:re?.y??0,sourceAlias:re?.source??``,targetAlias:re?.target??``,relations:re?.relations??[],onDeleteRelation:e=>{re&&D?.([{source:re.source,target:re.target,relation:e}])},onClose:()=>j(null)})]})]})},Oe)}var Yo={root:`_root_1yhjs_2`,empty:`_empty_1yhjs_10`,emptyIcon:`_emptyIcon_1yhjs_23`,toolbarButton:`_toolbarButton_1yhjs_29 _toolbarButton_stok4_116`,scrollBody:`_scrollBody_1yhjs_34`,jsonContainer:`_jsonContainer_1yhjs_45`,jsonLabel:`_jsonLabel_1yhjs_46`,jsonString:`_jsonString_1yhjs_47`,jsonNumber:`_jsonNumber_1yhjs_48`,jsonBoolean:`_jsonBoolean_1yhjs_49`,jsonNull:`_jsonNull_1yhjs_50`},Xo={default:e=>e<3,all:i,none:a};function Zo({graphData:e,graphName:t,onCopySuccess:n,onCopyError:i,onDownload:a}){let[s,c]=(0,P.useState)(`all`);return e?(0,z.jsxs)(`div`,{className:Yo.root,children:[(0,z.jsx)(ho,{graphData:e,graphName:t,onCopySuccess:n,onCopyError:i,onDownload:a,extraActions:(0,z.jsxs)(z.Fragment,{children:[(0,z.jsx)(`button`,{className:Yo.toolbarButton,onClick:()=>c(`all`),title:`Expand all nodes`,"aria-label":`Expand all JSON nodes`,"aria-pressed":s===`all`,children:`➖`}),(0,z.jsx)(`button`,{className:Yo.toolbarButton,onClick:()=>c(`none`),title:`Collapse all nodes`,"aria-label":`Collapse all JSON nodes`,"aria-pressed":s===`none`,children:`➕`})]})}),(0,z.jsx)(`div`,{className:Yo.scrollBody,children:(0,z.jsx)(o,{data:e,shouldExpandNode:Xo[s],style:{...r,container:`${r.container} ${Yo.jsonContainer}`,label:Yo.jsonLabel,stringValue:Yo.jsonString,numberValue:Yo.jsonNumber,booleanValue:Yo.jsonBoolean,nullValue:Yo.jsonNull}})})]}):(0,z.jsx)(`div`,{className:Yo.root,children:(0,z.jsxs)(`div`,{className:Yo.empty,children:[(0,z.jsx)(`span`,{className:Yo.emptyIcon,children:`🕸️`}),(0,z.jsx)(`span`,{children:`No graph data yet.`}),(0,z.jsx)(`span`,{children:`The raw JSON follows the live session graph.`})]})})}var Qo={rightPanel:`_rightPanel_xa3j1_2`,tabStrip:`_tabStrip_xa3j1_10`,tab:`_tab_xa3j1_10`,tabActive:`_tabActive_xa3j1_41`,tabBadge:`_tabBadge_xa3j1_45`,tabBody:`_tabBody_xa3j1_51`,tabBodyHidden:`_tabBodyHidden_xa3j1_64`,graphContent:`_graphContent_xa3j1_68`,rightPanelGroup:`_rightPanelGroup_xa3j1_75`,verticalResizeHandle:`_verticalResizeHandle_xa3j1_83`},$o=`help-split-percent`,es=`help-split-maximized`,ts=45,ns=98;function rs({tabs:e,payload:t,onChange:n,validation:r,onFormat:i,onUpload:a,graphData:o,graphName:s,activeTab:c,onTabChange:l,onGraphRenderError:u,onGraphDataCopySuccess:d,onGraphDataCopyError:f,graphRunControls:p,isGraphRefreshing:m,onClipNode:h,onClipNodes:g,onClipboardDrop:_,onImportFiles:v,onImportGraph:y,onDownloadGraph:b,isConnected:x,supportsAuthoring:S,onCreateNode:C,onCreateConnection:w,onEditNode:ee,onDeleteNode:T,onDeleteNodes:E,onDeleteConnections:D,panelLayoutKey:O,helpPanel:k}){let re=(0,P.useId)(),j=`${re}-tab-payload`,M=`${re}-tab-graph`,N=`${re}-tab-graph-data`,ie=!!k,F=(0,P.useRef)(Number(sessionStorage.getItem($o))||ts),ae=(0,P.useRef)(null),I=(0,P.useRef)(null),[L,oe]=(0,P.useState)(()=>sessionStorage.getItem(es)===`1`),se=(0,P.useRef)(L),ce=(0,z.jsxs)(`div`,{className:Qo.rightPanel,children:[e.length>1&&(0,z.jsxs)(`div`,{className:Qo.tabStrip,role:`tablist`,"aria-label":`Right panel tabs`,children:[e.includes(`payload`)&&(0,z.jsx)(`button`,{role:`tab`,"aria-selected":c===`payload`,"aria-controls":j,className:`${Qo.tab}${c===`payload`?` ${Qo.tabActive}`:``}`,onClick:()=>l(`payload`),children:`Payload Editor`}),e.includes(`graph`)&&(0,z.jsxs)(`button`,{role:`tab`,"aria-selected":c===`graph`,"aria-controls":M,className:`${Qo.tab}${c===`graph`?` ${Qo.tabActive}`:``}`,onClick:()=>l(`graph`),children:[`Graph`,o!==null&&(0,z.jsx)(`span`,{className:Qo.tabBadge,"aria-label":`Graph data available`,children:`🕸️`})]}),e.includes(`graph-data`)&&(0,z.jsx)(`button`,{role:`tab`,"aria-selected":c===`graph-data`,"aria-controls":N,className:`${Qo.tab}${c===`graph-data`?` ${Qo.tabActive}`:``}`,onClick:()=>l(`graph-data`),children:`Raw`})]}),e.includes(`payload`)&&(0,z.jsx)(`div`,{role:`tabpanel`,id:j,tabIndex:c===`payload`?0:-1,className:`${Qo.tabBody}${c===`payload`?``:` ${Qo.tabBodyHidden}`}`,children:(0,z.jsx)(ii,{payload:t,onChange:n,validation:r,onFormat:i,onUpload:a})}),e.includes(`graph`)&&(0,z.jsx)(`div`,{role:`tabpanel`,id:M,tabIndex:c===`graph`?0:-1,className:`${Qo.tabBody}${c===`graph`?``:` ${Qo.tabBodyHidden}`}`,children:(0,z.jsx)(`div`,{className:Qo.graphContent,children:(0,z.jsx)(Jo,{graphData:o,graphName:s,onRenderError:u,isRefreshing:m,onCopySuccess:d,onCopyError:f,graphRunControls:p,onClipNode:h,onClipNodes:g,onClipboardDrop:_,onImportFiles:v,onImportGraph:y,onDownload:b,isActive:c===`graph`,isConnected:x,supportsAuthoring:S,onCreateNode:C,onCreateConnection:w,onEditNode:ee,onDeleteNode:T,onDeleteNodes:E,onDeleteConnections:D,panelLayoutKey:O})})}),e.includes(`graph-data`)&&(0,z.jsx)(`div`,{role:`tabpanel`,id:N,tabIndex:c===`graph-data`?0:-1,className:`${Qo.tabBody}${c===`graph-data`?``:` ${Qo.tabBodyHidden}`}`,children:(0,z.jsx)(Zo,{graphData:o,graphName:s,onCopySuccess:d,onCopyError:f,onDownload:b})})]}),le=(0,P.useCallback)(e=>{let t=e[`help-split-help`];if(t===void 0)return;let n=t>=ns;n!==se.current&&(se.current=n,oe(n),sessionStorage.setItem(es,n?`1`:`0`)),n||(F.current=t,sessionStorage.setItem($o,String(t)))},[]),ue=(0,P.useCallback)(()=>{let e=!se.current;if(se.current=e,oe(e),sessionStorage.setItem(es,e?`1`:`0`),e)I.current?.resize(`0%`),ae.current?.resize(`100%`);else{let e=F.current;ae.current?.resize(`${e}%`),I.current?.resize(`${100-e}%`)}},[]);if((0,P.useEffect)(()=>{ie&&se.current&&requestAnimationFrame(()=>{I.current?.resize(`0%`),ae.current?.resize(`100%`)})},[ie]),!k)return ce;let R=typeof k==`function`?k(ue,L):k,de=se.current?100:F.current,fe=100-de;return(0,z.jsxs)(ne,{orientation:`vertical`,className:Qo.rightPanelGroup,onLayoutChanged:le,children:[(0,z.jsx)(A,{panelRef:I,defaultSize:`${fe}%`,minSize:`0%`,children:ce}),(0,z.jsx)(te,{className:Qo.verticalResizeHandle,"aria-label":`Resize help panel`}),(0,z.jsx)(A,{id:`help-split-help`,panelRef:ae,defaultSize:`${de}%`,minSize:`15%`,children:R})]})}var is=class extends P.Component{constructor(...e){super(...e),this.state={hasError:!1}}static getDerivedStateFromError(){return{hasError:!0}}componentDidCatch(e,t){console.error(`[ConsoleErrorBoundary] Failed to render message:`,e,t.componentStack)}render(){return this.state.hasError?(0,z.jsx)(`span`,{children:this.props.fallback}):this.props.children}},as=2e3,os=(e={})=>{let{onSuccess:t,onError:n}=e,[r,i]=(0,P.useState)(!1),a=(0,P.useRef)(null);return(0,P.useEffect)(()=>()=>{a.current!==null&&clearTimeout(a.current)},[]),{copy:(0,P.useCallback)(async e=>{if(!navigator.clipboard)return console.warn(`useCopyToClipboard: Clipboard API not available in this browser.`),n?.(),!1;try{return await navigator.clipboard.writeText(e),i(!0),a.current!==null&&clearTimeout(a.current),a.current=setTimeout(()=>{a.current=null,i(!1)},as),t?.(),!0}catch(e){return console.error(`useCopyToClipboard: Failed to write to clipboard.`,e),n?.(),!1}},[t,n]),copied:r}},W={consoleRoot:`_consoleRoot_1lgp1_2`,consoleHeader:`_consoleHeader_1lgp1_10`,consoleTitle:`_consoleTitle_1lgp1_20`,consoleControls:`_consoleControls_1lgp1_25`,controlButton:`_controlButton_1lgp1_30`,console:`_console_1lgp1_2`,emptyConsole:`_emptyConsole_1lgp1_67`,consoleMessage:`_consoleMessage_1lgp1_80`,consoleMessageActivatable:`_consoleMessageActivatable_1lgp1_94`,consoleMessageGraphLink:`_consoleMessageGraphLink_1lgp1_104`,consoleMessageLargePayload:`_consoleMessageLargePayload_1lgp1_115`,consoleMessageMockUpload:`_consoleMessageMockUpload_1lgp1_122`,uploadMockButton:`_uploadMockButton_1lgp1_131`,copyButton:`_copyButton_1lgp1_172`,copyButtonCopied:`_copyButtonCopied_1lgp1_225`,sendToJsonPathButton:`_sendToJsonPathButton_1lgp1_234`,messageIcon:`_messageIcon_1lgp1_268`,messageContent:`_messageContent_1lgp1_272`,messageText:`_messageText_1lgp1_278`,messageTime:`_messageTime_1lgp1_283`,"messageType-error":`_messageType-error_1lgp1_290`,"messageType-info":`_messageType-info_1lgp1_291`,"messageType-welcome":`_messageType-welcome_1lgp1_292`,jsonViewWrapper:`_jsonViewWrapper_1lgp1_295`,jsonContainer:`_jsonContainer_1lgp1_301`,jsonLabel:`_jsonLabel_1lgp1_302`,jsonString:`_jsonString_1lgp1_303`,jsonNumber:`_jsonNumber_1lgp1_304`,jsonBoolean:`_jsonBoolean_1lgp1_305`,jsonNull:`_jsonNull_1lgp1_306`};function ss({message:e,msgId:t,classificationMap:n,onGraphLink:i,onCopyMessage:a,onSendToJsonPath:s,onUploadMockData:c,successfulUploadPaths:l}){let u=_e(e),d=ve(u.type),f=ye(u.message),p=(t===void 0?void 0:n?.get(t))??[],m=p.some(e=>e.kind===`graph.link`),h=p.some(e=>e.kind===`payload.large`),g=p.some(e=>e.kind===`upload.invitation`),_=p.find(e=>e.kind===`upload.invitation`)?.uploadPath??null,v=!!c&&g&&_!==null,y=v&&!!l?.has(_),b=!!i&&m&&!g&&!h,x=!!s&&f.isJSON,{copy:S,copied:C}=os({onSuccess:a}),w=t=>{t.stopPropagation(),S(e)},ee=t=>{(t.key===`Enter`||t.key===` `)&&(t.preventDefault(),t.stopPropagation(),S(e))},T=e=>{e.stopPropagation(),!(!s||!f.isJSON)&&s(JSON.stringify(f.data,null,2))},E=e=>{e.stopPropagation(),!(!c||!_)&&c(_)};return(0,z.jsxs)(`div`,{className:[W.consoleMessage,W[`messageType-${u.type}`],b?W.consoleMessageActivatable:``,m?W.consoleMessageGraphLink:``,h?W.consoleMessageLargePayload:``,g?W.consoleMessageMockUpload:``].filter(Boolean).join(` `),onClick:b?()=>i():void 0,title:b?`Click to load graph in Graph View`:void 0,role:b?`button`:void 0,tabIndex:b?0:void 0,onKeyDown:b?e=>{(e.key===`Enter`||e.key===` `)&&(e.preventDefault(),i())}:void 0,"aria-label":b?`Load graph in Graph View`:void 0,children:[(0,z.jsx)(`span`,{className:W.messageIcon,children:g?`⬆️`:h?`⬇️`:m?`🕸️`:d}),(0,z.jsx)(`div`,{className:W.messageContent,children:f.isJSON?(0,z.jsx)(`div`,{className:W.jsonViewWrapper,children:(0,z.jsx)(o,{data:f.data,shouldExpandNode:e=>e<1,style:{...r,container:`${r.container} ${W.jsonContainer}`,label:W.jsonLabel,stringValue:W.jsonString,numberValue:W.jsonNumber,booleanValue:W.jsonBoolean,nullValue:W.jsonNull}})}):(0,z.jsxs)(`span`,{className:W.messageText,children:[u.message,y&&(0,z.jsx)(`span`,{title:`Upload succeeded`,children:` ✅`})]})}),(0,z.jsx)(`button`,{className:`${W.copyButton} ${C?W.copyButtonCopied:``}`,onClick:w,onKeyDown:ee,title:C?`Copied!`:`Copy message`,"aria-label":C?`Copied to clipboard`:`Copy message to clipboard`,tabIndex:0,children:C?`✅`:`📄`}),x&&(0,z.jsx)(`button`,{className:W.sendToJsonPathButton,onClick:T,onKeyDown:e=>{(e.key===`Enter`||e.key===` `)&&T(e)},title:`Open in JSON-Path Playground`,"aria-label":`Open this JSON in the JSON-Path Playground`,tabIndex:0,children:`➡️`}),v&&(0,z.jsx)(`button`,{className:W.uploadMockButton,onClick:E,onKeyDown:e=>{(e.key===`Enter`||e.key===` `)&&E(e)},title:`Re-open upload dialog`,"aria-label":`Re-open mock data upload dialog`,tabIndex:0,children:`⬆️ Upload JSON…`}),u.time&&(0,z.jsx)(`span`,{className:W.messageTime,children:u.time})]})}function cs({messages:e,classificationMap:t,onCopy:n,onClear:r,consoleRef:i,onGraphLinkMessage:a,onCopyMessage:o,onSendToJsonPath:s,onUploadMockData:c,successfulUploadPaths:l}){return(0,z.jsxs)(`div`,{className:W.consoleRoot,children:[(0,z.jsxs)(`div`,{className:W.consoleHeader,children:[(0,z.jsx)(`span`,{className:W.consoleTitle,children:`Console Output`}),(0,z.jsxs)(`div`,{className:W.consoleControls,children:[(0,z.jsx)(`button`,{className:W.controlButton,onClick:n,title:`Copy console output`,"aria-label":`Copy console output to clipboard`,children:`📑`}),(0,z.jsx)(`button`,{className:W.controlButton,onClick:r,title:`Clear console`,"aria-label":`Clear console`,children:`🗑️`})]})]}),(0,z.jsxs)(`div`,{className:W.console,ref:i,role:`log`,"aria-live":`polite`,children:[e.map(e=>(0,z.jsx)(is,{fallback:e.raw,children:(0,z.jsx)(ss,{message:e.raw,msgId:e.id,classificationMap:t,onGraphLink:a?()=>a(e):void 0,onCopyMessage:o,onSendToJsonPath:s,onUploadMockData:c,successfulUploadPaths:l})},e.id)),e.length===0&&(0,z.jsxs)(`div`,{className:W.emptyConsole,children:[`No messages yet. Use the `,(0,z.jsx)(`strong`,{children:`Start`}),` button in the header to connect.`]})]})]})}var ls={commandInput:`_commandInput_188pe_2`,labelRow:`_labelRow_188pe_8`,labelGroup:`_labelGroup_188pe_16`,label:`_label_188pe_8`,infoWrapper:`_infoWrapper_188pe_28`,paletteToggle:`_paletteToggle_188pe_34`,paletteToggleActive:`_paletteToggleActive_188pe_66`,popover:`_popover_188pe_73`,popoverOpen:`_popoverOpen_188pe_95`,popoverTitle:`_popoverTitle_188pe_121`,popoverRow:`_popoverRow_188pe_135`,popoverKeyword:`_popoverKeyword_188pe_156`,popoverDesc:`_popoverDesc_188pe_168`,popoverAlias:`_popoverAlias_188pe_174`,inputRow:`_inputRow_188pe_181`,inputWrapper:`_inputWrapper_188pe_187`,textarea:`_textarea_188pe_197`,sendButton:`_sendButton_188pe_236`,hint:`_hint_188pe_253`,dropup:`_dropup_188pe_261`,dropupHeader:`_dropupHeader_188pe_276`,dropupItem:`_dropupItem_188pe_292`,dropupItemText:`_dropupItemText_188pe_315`,matchHighlight:`_matchHighlight_188pe_323`,multilineIndicator:`_multilineIndicator_188pe_329`},us=[`graph.data.mapper`,`graph.math`,`graph.js`,`graph.api.fetcher`,`graph.extension`,`graph.island`,`graph.join`],ds=[{keyword:`help`,description:`List all help topics, or get help for a specific command`,template:`help`},{keyword:`create`,description:`Create a new graph node`,template:`create node {name}
with type {type}
with properties
{key}={value}`,multiline:!0},{keyword:`update`,description:`Update an existing node`,template:`update node {name}
with type {type}
with properties
{key}={value}`,multiline:!0},{keyword:`edit`,description:`Print raw node data ready for editing and re-submitting`,template:`edit node {name}`},{keyword:`delete node`,description:`Delete a node by name`,alias:`clear node`,template:`delete node {name}`},{keyword:`delete connection`,description:`Delete connection(s) between two nodes`,alias:`clear connection`,template:`delete connection {nodeA} and {nodeB}`},{keyword:`delete cache`,description:`Clear cached API fetcher results`,alias:`clear cache`,template:`delete cache`},{keyword:`connect`,description:`Connect two nodes with a named relation`,template:`connect {node-A} to {node-B} with {relation}`},{keyword:`list nodes`,description:`List all nodes in the current graph`,template:`list nodes`},{keyword:`list connections`,description:`List all connections in the current graph`,template:`list connections`},{keyword:`describe graph`,description:`Describe the current graph model`,template:`describe graph`},{keyword:`describe node`,description:`Describe a specific node and its connections`,template:`describe node {name}`},{keyword:`describe connection`,description:`Describe connection(s) between two nodes`,template:`describe connection {nodeA} and {nodeB}`},{keyword:`describe skill`,description:`Show documentation for a skill by route name`,template:`describe skill {skill.route}`},{keyword:`export`,description:`Export the graph model to a JSON file`,template:`export graph as {name}`},{keyword:`import graph`,description:`Import a graph model from a saved file`,template:`import graph from {name}`},{keyword:`import node`,description:`Import a single node from another saved graph`,template:`import node {node-name} from {graph-name}`},{keyword:`instantiate`,description:`Create a runnable graph instance with mock input`,alias:`start`,template:`instantiate graph
{constant} -> input.body.{key}`,multiline:!0},{keyword:`upload mock data`,description:`Print the URL to POST a JSON payload as mock input.body`,template:`upload mock data`},{keyword:`execute`,description:`Execute a single node skill in isolation`,template:`execute node {name}`},{keyword:`inspect`,description:`Inspect a state-machine variable`,template:`inspect {variable_name}`},{keyword:`run`,description:`Run the graph instance from root to end`,template:`run`}];[...us.map(e=>({tokens:[`describe`,`skill`,e],template:`describe skill ${e}`,hint:`Describe built-in skill: ${e}`}))];function fs(e,t){let[n,r]=(0,P.useState)(!1),[i,a]=(0,P.useState)(-1),o=(0,P.useMemo)(()=>{let n=t.trimStart();if(n.length===0)return[];let r=n.toLowerCase(),i=e.filter(e=>e.toLowerCase().startsWith(r)),a=new Set;return i.filter(e=>a.has(e)?!1:(a.add(e),!0)).slice(0,8)},[e,t]),s=()=>{r(!0),a(-1)},c=e=>{let t=o.length;t!==0&&a(n=>e===1?n<0?0:(n+1)%t:n<=0?t-1:n-1)},l=(e,t)=>{e>=0&&e<o.length&&t(o[e]),r(!1),a(-1)};return{suggestions:o,isOpen:n,activeIndex:i,onCommandChange:s,navigate:c,accept:l,onTab:e=>{!n||o.length===0||l(i>=0?i:0,e)},dismiss:()=>{r(!1),a(-1)}}}var ps=e=>(0,z.jsxs)(`svg`,{xmlns:`http://www.w3.org/2000/svg`,viewBox:`0 0 16 16`,fill:`none`,width:14,height:14,stroke:`currentColor`,strokeWidth:1.5,strokeLinecap:`round`,strokeLinejoin:`round`,...e,children:[(0,z.jsx)(`polyline`,{points:`2,4 6,8 2,12`}),(0,z.jsx)(`line`,{x1:7,y1:12,x2:14,y2:12})]});function ms({command:e,onChange:t,onKeyDown:n,onSend:r,sendDisabled:i,disabled:a,history:o}){let s=(0,P.useRef)(null),c=(0,P.useRef)(null),l=(0,P.useRef)(null),[u,d]=(0,P.useState)(!1);(0,P.useEffect)(()=>{if(!u)return;let e=e=>{c.current&&!c.current.contains(e.target)&&d(!1)};return document.addEventListener(`mousedown`,e),()=>document.removeEventListener(`mousedown`,e)},[u]);let f=fs(o,e),p=(0,P.useCallback)(()=>{let e=s.current;e&&(e.style.height=`auto`,e.style.height=`${e.scrollHeight}px`)},[]);(0,P.useEffect)(()=>{p()},[e,p]),(0,P.useEffect)(()=>{let e=s.current?.parentElement;if(!e||typeof ResizeObserver>`u`)return;let t=null,n=new ResizeObserver(()=>{t===null&&(t=requestAnimationFrame(()=>{t=null,p()}))});return n.observe(e),()=>{n.disconnect(),t!==null&&cancelAnimationFrame(t)}},[p]);let m=a?`Not connected`:`Enter command (Enter to send · Shift+Enter for new line)`,h=a?`Enter your test message once it is connected`:`Enter to send · Shift+Enter for new line · ↑↓ for history`;return(0,z.jsxs)(`div`,{className:ls.commandInput,children:[(0,z.jsx)(`div`,{className:ls.labelRow,children:(0,z.jsxs)(`div`,{className:ls.labelGroup,children:[(0,z.jsx)(`label`,{htmlFor:`command`,className:ls.label,children:`Command`}),(0,z.jsxs)(`span`,{ref:c,className:ls.infoWrapper,children:[(0,z.jsx)(`button`,{type:`button`,className:`${ls.paletteToggle}${u?` ${ls.paletteToggleActive}`:``}`,"aria-label":`Toggle command palette`,"aria-expanded":u,"aria-controls":`command-palette`,onClick:()=>d(e=>!e),onKeyDown:e=>{e.key===`ArrowDown`&&u&&(e.preventDefault(),(l.current?.querySelector(`[role="option"]`))?.focus())},title:`Command palette`,children:(0,z.jsx)(ps,{"aria-hidden":`true`,focusable:`false`})}),(0,z.jsxs)(`div`,{id:`command-palette`,ref:l,className:`${ls.popover}${u?` ${ls.popoverOpen}`:``}`,role:`listbox`,"aria-label":`Command palette`,onKeyDown:e=>{if(e.key===`ArrowDown`||e.key===`ArrowUp`){e.preventDefault();let t=l.current?.querySelectorAll(`[role="option"]`);if(!t||t.length===0)return;let n=Array.from(t).indexOf(document.activeElement);e.key===`ArrowDown`?t[n<0?0:(n+1)%t.length].focus():t[n<=0?t.length-1:n-1].focus()}else e.key===`Escape`&&(e.preventDefault(),d(!1),s.current?.focus())},children:[(0,z.jsx)(`p`,{className:ls.popoverTitle,children:`Command palette — click to insert`}),ds.map(({keyword:e,alias:n,description:r,template:i})=>(0,z.jsxs)(`div`,{className:ls.popoverRow,role:`option`,"aria-selected":!1,tabIndex:u?0:-1,onMouseDown:e=>e.preventDefault(),onClick:()=>{t(i),d(!1),s.current?.focus()},onKeyDown:e=>{(e.key===`Enter`||e.key===` `)&&(e.preventDefault(),t(i),d(!1),s.current?.focus())},children:[(0,z.jsx)(`span`,{className:ls.popoverKeyword,children:e}),(0,z.jsxs)(`span`,{className:ls.popoverDesc,children:[r,n&&(0,z.jsxs)(`span`,{className:ls.popoverAlias,children:[` · alias: `,n]})]})]},e))]})]})]})}),(0,z.jsxs)(`div`,{className:ls.inputRow,children:[(0,z.jsxs)(`div`,{className:ls.inputWrapper,children:[(0,z.jsxs)(`div`,{id:`history-dropup`,role:`listbox`,"aria-label":`Command history suggestions`,className:ls.dropup,hidden:!(f.isOpen&&f.suggestions.length>0),children:[(0,z.jsx)(`div`,{className:ls.dropupHeader,"aria-hidden":`true`,children:`Recent Commands`}),f.isOpen&&f.suggestions.length>0&&f.suggestions.map((n,r)=>{let i=n.split(`
`)[0],a=n.includes(`
`),o=e.trimStart().split(`
`)[0],c=Math.min(o.length,i.length),l=i.slice(0,c),u=i.slice(c);return(0,z.jsxs)(`div`,{id:`history-option-${r}`,role:`option`,"aria-selected":r===f.activeIndex,className:ls.dropupItem,onMouseDown:e=>e.preventDefault(),onClick:()=>{f.accept(r,e=>t(e)),requestAnimationFrame(()=>{let e=s.current;e&&(e.selectionStart=e.selectionEnd=e.value.length)})},children:[(0,z.jsxs)(`span`,{className:ls.dropupItemText,children:[c>0&&(0,z.jsx)(`strong`,{className:ls.matchHighlight,children:l}),u,a?`…`:``]}),a&&(0,z.jsx)(`span`,{className:ls.multilineIndicator,"aria-label":`multi-line command`,children:`↵`})]},n)})]}),(0,z.jsx)(`textarea`,{ref:s,id:`command`,role:`combobox`,"aria-expanded":f.isOpen&&f.suggestions.length>0,"aria-haspopup":`listbox`,"aria-controls":`history-dropup`,"aria-activedescendant":f.isOpen&&f.suggestions.length>0&&f.activeIndex>=0?`history-option-${f.activeIndex}`:void 0,"aria-autocomplete":`list`,className:ls.textarea,rows:1,placeholder:m,value:e,disabled:a,onChange:e=>{t(e.target.value),f.onCommandChange()},onKeyDown:e=>{if(e.key===`Tab`){e.preventDefault(),f.isOpen&&f.suggestions.length>0&&(f.onTab(e=>t(e)),requestAnimationFrame(()=>{let e=s.current;e&&(e.selectionStart=e.selectionEnd=e.value.length)}));return}if(e.key===`Enter`){if(e.shiftKey)return;if(e.preventDefault(),f.isOpen&&f.activeIndex>=0){f.accept(f.activeIndex,e=>t(e)),requestAnimationFrame(()=>{let e=s.current;e&&(e.selectionStart=e.selectionEnd=e.value.length)}),s.current?.focus();return}r(),s.current?.focus();return}if(e.key===`Escape`){if(f.isOpen){f.dismiss(),e.preventDefault();return}return}if(e.key===`ArrowUp`||e.key===`ArrowDown`){if(f.isOpen&&f.suggestions.length>0){e.preventDefault(),f.navigate(e.key===`ArrowDown`?1:-1);return}let t=s.current;if(t){let{selectionStart:n,value:r}=t,i=!r.slice(0,n).includes(`
`),a=!r.slice(n).includes(`
`);if(!(e.key===`ArrowUp`&&i||e.key===`ArrowDown`&&a))return}n(e),requestAnimationFrame(()=>{let e=s.current;e&&(e.selectionStart=e.selectionEnd=e.value.length)});return}n(e)},onBlur:()=>f.dismiss(),autoComplete:`off`,autoCorrect:`off`,spellCheck:!1})]}),(0,z.jsx)(`button`,{className:ls.sendButton,onClick:()=>{r(),s.current?.focus()},disabled:i,"aria-label":`Send command`,children:`Send`})]}),h&&(0,z.jsx)(`p`,{className:ls.hint,children:h})]})}var hs={root:`_root_1ac49_1`};function gs({messages:e,classificationMap:t,onCopy:n,onClear:r,consoleRef:i,onGraphLinkMessage:a,onCopyMessage:o,onSendToJsonPath:s,onUploadMockData:c,successfulUploadPaths:l,command:u,onCommandChange:d,onCommandKeyDown:f,onSend:p,sendDisabled:m,inputDisabled:h,commandHistory:g}){return(0,z.jsxs)(`div`,{className:hs.root,children:[(0,z.jsx)(cs,{messages:e,classificationMap:t,onCopy:n,onClear:r,consoleRef:i,onGraphLinkMessage:a,onCopyMessage:o,onSendToJsonPath:s,onUploadMockData:c,successfulUploadPaths:l}),(0,z.jsx)(ms,{command:u,onChange:d,onKeyDown:f,onSend:p,disabled:h,sendDisabled:m,history:g})]})}var _s=e=>(0,z.jsxs)(`svg`,{xmlns:`http://www.w3.org/2000/svg`,viewBox:`0 0 16 16`,fill:`none`,width:16,height:16,stroke:`currentColor`,strokeWidth:1.8,strokeLinecap:`round`,strokeLinejoin:`round`,...e,children:[(0,z.jsx)(`line`,{x1:4.75,y1:4.75,x2:11.25,y2:11.25}),(0,z.jsx)(`line`,{x1:11.25,y1:4.75,x2:4.75,y2:11.25})]}),G={root:`_root_1f65i_9`,card:`_card_1f65i_19`,ribbon:`_ribbon_1f65i_32`,ribbonIcon:`_ribbonIcon_1f65i_45`,ribbonAlias:`_ribbonAlias_1f65i_50`,ribbonAliasInput:`_ribbonAliasInput_1f65i_57`,ribbonBadge:`_ribbonBadge_1f65i_82`,ribbonClose:`_ribbonClose_1f65i_93`,ribbonCloseIcon:`_ribbonCloseIcon_1f65i_114`,body:`_body_1f65i_121`,row:`_row_1f65i_128`,rowDropTarget:`_rowDropTarget_1f65i_138`,dragGrip:`_dragGrip_1f65i_142`,rowLabel:`_rowLabel_1f65i_166`,rowKey:`_rowKey_1f65i_167`,rowValue:`_rowValue_1f65i_178`,rowSpacer:`_rowSpacer_1f65i_185`,keyInput:`_keyInput_1f65i_189`,valueInput:`_valueInput_1f65i_190`,removeButton:`_removeButton_1f65i_228`,removeIcon:`_removeIcon_1f65i_251`,addRow:`_addRow_1f65i_257`,addButton:`_addButton_1f65i_263`,message:`_message_1f65i_282`,errorMessage:`_errorMessage_1f65i_283`,warningMessage:`_warningMessage_1f65i_284`,errorText:`_errorText_1f65i_306`,footer:`_footer_1f65i_312`,secondaryButton:`_secondaryButton_1f65i_321`,primaryButton:`_primaryButton_1f65i_322`,schemaLink:`_schemaLink_1f65i_359`},vs=1,ys=10,bs=36;function xs(e){let t=e.split(`
`).reduce((e,t)=>e+Math.max(1,Math.ceil(t.length/bs)),0);return Math.min(Math.max(t,vs),ys)}function Ss({mode:e,formState:t,phase:n,lockReason:r,serverMessage:i,validationErrors:a,onFormStateChange:o,onSubmit:s,onClose:c,onOpenSchemaPanel:l}){let u=(0,P.useRef)(null),d=(0,P.useRef)(null),f=(0,P.useRef)(new Map),p=(0,P.useRef)(null),m=e===`create`,h=n===`sending`,g=r===`disconnected`,_=h||g,v=m?`Create Node`:`Save Changes`,y=m?`Creating...`:`Saving...`,b=m?`Connection disconnected. Refresh the page and create the node again after the app reconnects.`:`Connection disconnected. Refresh the page and edit the node again after the app reconnects.`,x=li(t.nodeType),S=ui(t.nodeType);(0,P.useEffect)(()=>{m?u.current?.focus():d.current?.focus();let e=e=>{e.key===`Escape`&&(e.preventDefault(),h||c())};return document.addEventListener(`keydown`,e),()=>{document.removeEventListener(`keydown`,e)}},[m,c,h]),(0,P.useEffect)(()=>{let e=p.current;if(!e)return;let t=f.current.get(e);t&&(t.focus(),p.current=null)},[t.properties]);let C=(0,P.useCallback)(e=>{e.preventDefault(),!_&&s()},[_,s]),w=(0,P.useCallback)(e=>{o({...t,...e})},[t,o]),ee=(0,P.useCallback)((e,n)=>{o({...t,properties:t.properties.map(t=>t.id===e?{...t,...n}:t)})},[t,o]),T=(0,P.useCallback)(()=>{let e=er();p.current=e.id,o({...t,properties:[...t.properties,e]})},[t,o]),E=(0,P.useCallback)(e=>{let n=t.properties.filter(t=>t.id!==e);o({...t,properties:n.length>0?n:[er()]})},[t,o]),D=(0,P.useRef)(null),[O,k]=(0,P.useState)(null),A=(0,P.useCallback)(()=>{D.current=null,k(null)},[]),te=(0,P.useCallback)(e=>t=>{if(D.current=e,t.dataTransfer){t.dataTransfer.effectAllowed=`move`,t.dataTransfer.setData(`text/plain`,e);let n=t.currentTarget.closest(`[data-row-id]`);n instanceof HTMLElement&&typeof t.dataTransfer.setDragImage==`function`&&t.dataTransfer.setDragImage(n,16,16)}},[]),ne=(0,P.useCallback)(e=>t=>{D.current!==null&&(t.preventDefault(),t.dataTransfer&&(t.dataTransfer.dropEffect=`move`),k(t=>t===e?t:e))},[]),re=(0,P.useCallback)(e=>{let n=D.current;if(A(),n===null||n===e)return;let r=t.properties.find(e=>e.id===n);if(!r)return;let i=t.properties.filter(e=>e.id!==n),a=i.length;if(e!==null){let t=i.findIndex(t=>t.id===e);t!==-1&&(a=t)}let s=[...i.slice(0,a),r,...i.slice(a)];o({...t,properties:ar(s)})},[A,t,o]),j=(0,P.useCallback)(e=>t=>{D.current!==null&&(t.preventDefault(),re(e))},[re]);return(0,z.jsx)(`div`,{className:G.root,children:(0,z.jsxs)(`form`,{className:G.card,style:{borderColor:S,"--node-accent":S},"aria-label":m?`Create node`:`Edit node ${t.alias}`,onSubmit:C,children:[(0,z.jsxs)(`header`,{className:G.ribbon,children:[(0,z.jsx)(`span`,{className:G.ribbonIcon,"aria-hidden":`true`,children:x.icon}),m?(0,z.jsx)(`input`,{ref:u,className:G.ribbonAliasInput,value:t.alias,placeholder:`node-alias`,"aria-label":`Node alias`,disabled:_,"aria-invalid":!!a.alias,onChange:e=>w({alias:e.target.value})}):(0,z.jsx)(`span`,{className:G.ribbonAlias,children:t.alias}),(0,z.jsx)(`span`,{className:G.ribbonBadge,children:x.label}),(0,z.jsx)(`button`,{type:`button`,className:G.ribbonClose,"aria-label":`Close node editor`,title:`Close (Esc)`,onClick:c,disabled:h,children:(0,z.jsx)(_s,{className:G.ribbonCloseIcon,"aria-hidden":`true`,focusable:`false`})})]}),(0,z.jsxs)(`div`,{className:G.body,children:[i&&!g&&(0,z.jsx)(`div`,{className:G.message,role:`status`,children:i}),a.command&&(0,z.jsx)(`div`,{className:G.errorMessage,role:`alert`,children:a.command}),a.alias&&(0,z.jsx)(`div`,{className:G.errorMessage,role:`alert`,children:a.alias}),g&&(0,z.jsx)(`div`,{className:G.warningMessage,role:`status`,children:i??b}),l&&(0,z.jsxs)(`div`,{className:G.message,role:`note`,children:[`This node carries the graph's `,t.alias.trim()===`root`?`request`:`response`,` contract (`,(0,z.jsx)(`code`,{children:`schema`}),`).`,` `,(0,z.jsx)(`button`,{type:`button`,className:G.schemaLink,onClick:l,disabled:h,title:`Edit the contract as rows in the Schema panel; this editor closes`,children:`Open the Schema panel…`})]}),(0,z.jsxs)(`div`,{className:G.row,children:[(0,z.jsx)(`span`,{"aria-hidden":`true`}),(0,z.jsx)(`label`,{className:G.rowLabel,htmlFor:`node-edit-type`,children:`type`}),(0,z.jsxs)(`div`,{className:G.rowValue,children:[(0,z.jsx)(`input`,{id:`node-edit-type`,ref:d,className:G.valueInput,value:t.nodeType,disabled:_,"aria-invalid":!!a.nodeType,onChange:e=>w({nodeType:e.target.value})}),a.nodeType&&(0,z.jsx)(`span`,{className:G.errorText,children:a.nodeType})]}),(0,z.jsx)(`span`,{className:G.rowSpacer,"aria-hidden":`true`})]}),t.properties.map(e=>{let t=a[Wn(e.id,`key`)],n=a[Wn(e.id,`value`)];return(0,z.jsxs)(`div`,{"data-row-id":e.id,className:O===e.id?`${G.row} ${G.rowDropTarget}`:G.row,onDragOver:ne(e.id),onDrop:j(e.id),children:[(0,z.jsx)(`span`,{className:G.dragGrip,role:`button`,"aria-label":`Reorder property ${e.key.trim()||`(empty)`}`,title:`Drag to reorder — same keys append as [0], [1], … in row order`,draggable:!_,onDragStart:te(e.id),onDragEnd:A,children:`⠿`}),(0,z.jsxs)(`div`,{className:G.rowKey,children:[(0,z.jsx)(`input`,{ref:t=>{t?f.current.set(e.id,t):f.current.delete(e.id)},className:G.keyInput,value:e.key,placeholder:`key`,"aria-label":`Property key`,disabled:_,"aria-invalid":!!t,onChange:t=>ee(e.id,{key:t.target.value})}),t&&(0,z.jsx)(`span`,{className:G.errorText,children:t})]}),(0,z.jsxs)(`div`,{className:G.rowValue,children:[(0,z.jsx)(`textarea`,{className:G.valueInput,value:e.value,placeholder:`value`,"aria-label":`Property value`,disabled:_,rows:xs(e.value),"aria-invalid":!!n,onChange:t=>ee(e.id,{value:t.target.value})}),n&&(0,z.jsx)(`span`,{className:G.errorText,children:n})]}),(0,z.jsx)(`button`,{type:`button`,className:G.removeButton,"aria-label":`Remove property`,disabled:_,onClick:()=>E(e.id),children:(0,z.jsx)(_s,{className:G.removeIcon,"aria-hidden":`true`,focusable:`false`})})]},e.id)}),(0,z.jsx)(`div`,{className:O===`end`?`${G.addRow} ${G.rowDropTarget}`:G.addRow,onDragOver:ne(`end`),onDrop:j(null),children:(0,z.jsxs)(`button`,{type:`button`,className:G.addButton,disabled:_,onClick:T,children:[(0,z.jsx)(`span`,{"aria-hidden":`true`,children:`+`}),(0,z.jsx)(`span`,{children:`Add Property`})]})})]}),(0,z.jsxs)(`footer`,{className:G.footer,children:[(0,z.jsx)(`button`,{type:`button`,className:G.secondaryButton,onClick:c,disabled:h,children:`Cancel`}),(0,z.jsx)(`button`,{type:`submit`,className:G.primaryButton,disabled:_,children:h?y:v})]})]})})}var K={root:`_root_1bd9v_7`,card:`_card_1bd9v_17`,ribbon:`_ribbon_1bd9v_30`,titleGroup:`_titleGroup_1bd9v_41`,title:`_title_1bd9v_41`,path:`_path_1bd9v_54`,ribbonClose:`_ribbonClose_1bd9v_61`,ribbonCloseIcon:`_ribbonCloseIcon_1bd9v_90`,body:`_body_1bd9v_96`,description:`_description_1bd9v_105`,inputHints:`_inputHints_1bd9v_112`,inputHintsLabel:`_inputHintsLabel_1bd9v_122`,inputHintList:`_inputHintList_1bd9v_128`,moreHints:`_moreHints_1bd9v_135`,inputHintsNote:`_inputHintsNote_1bd9v_147`,dropZone:`_dropZone_1bd9v_153`,dropZoneActive:`_dropZoneActive_1bd9v_175`,dropZoneIcon:`_dropZoneIcon_1bd9v_181`,dropZoneText:`_dropZoneText_1bd9v_187`,dropZoneOr:`_dropZoneOr_1bd9v_200`,browseButton:`_browseButton_1bd9v_207`,fileInputHidden:`_fileInputHidden_1bd9v_236`,fileError:`_fileError_1bd9v_241`,textareaLabel:`_textareaLabel_1bd9v_246`,textarea:`_textarea_1bd9v_246`,validationError:`_validationError_1bd9v_274`,keyboardHint:`_keyboardHint_1bd9v_279`,errorBanner:`_errorBanner_1bd9v_284`,footer:`_footer_1bd9v_295`,footerActions:`_footerActions_1bd9v_305`,formatButton:`_formatButton_1bd9v_311`,cancelButton:`_cancelButton_1bd9v_312`,uploadButton:`_uploadButton_1bd9v_313`,spinner:`_spinner_1bd9v_380`,spin:`_spin_1bd9v_380`,rows:`_rows_1bd9v_396`,rowBlock:`_rowBlock_1bd9v_402`,row:`_row_1bd9v_396`,rowName:`_rowName_1bd9v_414`,rowValue:`_rowValue_1bd9v_418`,input:`_input_1bd9v_112`,inputInvalid:`_inputInvalid_1bd9v_435`,entryRemove:`_entryRemove_1bd9v_439`,entryIssue:`_entryIssue_1bd9v_444`,linkButton:`_linkButton_1bd9v_449`,headers:`_headers_1bd9v_474`};function Cs({uploadPath:e,json:t,headers:n,onSuccess:r,onError:i}){let[a,o]=(0,P.useState)(!1),s=(0,P.useRef)(null),c=(0,P.useCallback)(()=>{s.current?.abort(),s.current=null,o(!1)},[]);return{isUploading:a,upload:(0,P.useCallback)(async()=>{s.current?.abort();let a=new AbortController;s.current=a,o(!0);let c=Object.entries(n??{}),l=t.trim()!==``;try{let n=``;if(l){let r=await fetch(e,{method:`POST`,headers:{"Content-Type":`application/json`},body:t,signal:a.signal});if(n=await r.text(),!r.ok){o(!1),i(`HTTP ${r.status} — ${n}`);return}}if(c.length>0){let t=await fetch(`${e}?namespace=header`,{method:`POST`,headers:{"Content-Type":`application/json`},body:JSON.stringify(Object.fromEntries(c)),signal:a.signal});if(n=await t.text(),!t.ok){o(!1),i(`HTTP ${t.status} — ${n} (mock headers)`);return}}o(!1),r(n)}catch(e){if(e.name===`AbortError`){o(!1);return}o(!1),i(e.message??`Network error`)}},[e,t,n,r,i]),cancel:c}}function ws(e){return new Promise((t,n)=>{let r=new FileReader;r.onload=()=>t(r.result),r.onerror=()=>n(Error(`Could not read file "${e.name}"`)),r.readAsText(e,`utf-8`)})}function Ts(e){let t=e.name.toLowerCase().endsWith(`.json`),n=e.type===`application/json`||e.type===`text/plain`;return!t&&!n?`"${e.name}" does not appear to be a JSON file. Only .json files are accepted.`:null}var Es=0;function Ds(){return Es+=1,{key:`h${Es}`,name:``,value:``}}function Os(e){let t=new Map,n=new Set;for(let r of e){let e=r.name.trim();if(e===``){r.value.trim()!==``&&t.set(r.key,`A header value needs a name`);continue}let i=e.toLowerCase();n.has(i)?t.set(r.key,`Duplicate header name '${e}'`):n.add(i)}return t}var ks=(navigator.userAgentData?.platform??navigator.platform).toLowerCase().includes(`mac`);function As({uploadPath:e,onSuccess:t,onClose:n,onError:r,title:i=`⬆️ Upload Mock Data`,description:a,inputPathHints:o=[],inputHeaderHints:s=[],submitLabel:c=`Upload`}){let[l,u]=(0,P.useState)(``),[d,f]=(0,P.useState)([]),[p,m]=(0,P.useState)(null),[h,g]=(0,P.useState)(null),[_,v]=(0,P.useState)(!1),y=(0,P.useRef)(null),b=(0,P.useRef)(null),x=ye(l).isJSON,S=l.trim()!==``,C=(0,P.useMemo)(()=>Os(d),[d]),w=(0,P.useMemo)(()=>{let e={};for(let t of d){let n=t.name.trim();n!==``&&!C.has(t.key)&&(e[n]=t.value)}return e},[d,C]),ee=Object.keys(w).length,T=(S?x:ee>0)&&C.size===0,{isUploading:E,upload:D,cancel:O}=Cs({uploadPath:e,json:l,headers:w,onSuccess:n=>t(n,e),onError:e=>{m(e),r(e)}}),k=(0,P.useCallback)(()=>{O(),n(e)},[O,n,e]),A=(0,P.useRef)(E);(0,P.useEffect)(()=>{A.current=E},[E]),(0,P.useEffect)(()=>{y.current?.focus();let e=e=>{e.key===`Escape`&&(e.preventDefault(),A.current||k())};return document.addEventListener(`keydown`,e),()=>document.removeEventListener(`keydown`,e)},[k]);let te=(0,P.useCallback)(()=>{m(null),D()},[D]),ne=(0,P.useCallback)(e=>{e.key===`Enter`&&(e.ctrlKey||e.metaKey)&&(e.preventDefault(),T&&!E&&te())},[T,E,te]),re=(0,P.useCallback)(()=>f(e=>[...e,Ds()]),[]),j=(0,P.useCallback)((e,t)=>{f(n=>n.map(n=>n.key===e?{...n,...t}:n))},[]),M=(0,P.useCallback)(e=>{f(t=>t.filter(t=>t.key!==e))},[]),N=(0,P.useCallback)(()=>{x&&u(oe(l))},[x,l]),ie=(0,P.useCallback)(async e=>{g(null),m(null);let t=Ts(e);if(t){g(t);return}try{let t=await ws(e);if(!ye(t).isJSON){g(`"${e.name}" contains invalid JSON.`);return}u(oe(t)),y.current?.focus()}catch(e){g(e.message)}},[]),F=(0,P.useCallback)(e=>{e.preventDefault(),e.stopPropagation(),_||v(!0)},[_]),ae=(0,P.useCallback)(e=>{e.preventDefault(),e.stopPropagation(),(e.currentTarget===e.target||!e.currentTarget.contains(e.relatedTarget))&&v(!1)},[]),I=(0,P.useCallback)(e=>{e.preventDefault(),e.stopPropagation(),v(!1);let t=e.dataTransfer.files[0];t&&ie(t)},[ie]),L=(0,P.useCallback)(e=>{let t=e.target.files?.[0];t&&(ie(t),e.target.value=``)},[ie]),se=!x&&S;return(0,z.jsx)(`div`,{className:K.root,children:(0,z.jsxs)(`section`,{className:K.card,"aria-label":i,children:[(0,z.jsxs)(`header`,{className:K.ribbon,children:[(0,z.jsxs)(`div`,{className:K.titleGroup,children:[(0,z.jsx)(`span`,{className:K.title,children:i}),(0,z.jsx)(`span`,{className:K.path,children:e})]}),(0,z.jsx)(`button`,{className:K.ribbonClose,onClick:k,"aria-label":`Close upload panel`,title:`Close (Esc)`,disabled:E,children:(0,z.jsx)(_s,{className:K.ribbonCloseIcon,"aria-hidden":`true`,focusable:`false`})})]}),(0,z.jsxs)(`div`,{className:K.body,children:[a&&(0,z.jsx)(`p`,{className:K.description,children:a}),o.length>0&&(0,z.jsxs)(`div`,{className:K.inputHints,"aria-label":`Referenced graph input paths`,children:[(0,z.jsx)(`span`,{className:K.inputHintsLabel,children:`Referenced input paths`}),(0,z.jsxs)(`div`,{className:K.inputHintList,children:[o.slice(0,6).map(e=>(0,z.jsx)(`code`,{children:e},e)),o.length>6&&(0,z.jsxs)(`span`,{className:K.moreHints,children:[`+`,o.length-6,` more`]})]}),(0,z.jsx)(`span`,{className:K.inputHintsNote,children:`Hints are derived from graph references.`})]}),s.length>0&&(0,z.jsxs)(`div`,{className:K.inputHints,"aria-label":`Referenced input headers`,children:[(0,z.jsx)(`span`,{className:K.inputHintsLabel,children:`Referenced input headers`}),(0,z.jsxs)(`div`,{className:K.inputHintList,children:[s.slice(0,6).map(e=>(0,z.jsx)(`code`,{children:e===`*`?`input.header (all)`:e},e)),s.length>6&&(0,z.jsxs)(`span`,{className:K.moreHints,children:[`+`,s.length-6,` more`]})]}),(0,z.jsx)(`span`,{className:K.inputHintsNote,children:`Header names are case-insensitive to the graph.`})]}),(0,z.jsxs)(`div`,{className:`${K.dropZone} ${_?K.dropZoneActive:``}`,onDragOver:F,onDragLeave:ae,onDrop:I,"aria-label":`Drop a JSON file here`,children:[(0,z.jsx)(`span`,{className:K.dropZoneIcon,children:`📂`}),(0,z.jsxs)(`span`,{className:K.dropZoneText,children:[`Drop a `,(0,z.jsx)(`code`,{children:`.json`}),` file here`]}),(0,z.jsx)(`span`,{className:K.dropZoneOr,children:`— or —`}),(0,z.jsx)(`input`,{ref:b,type:`file`,accept:`.json,application/json`,className:K.fileInputHidden,"aria-hidden":`true`,tabIndex:-1,onChange:L}),(0,z.jsx)(`button`,{type:`button`,className:K.browseButton,onClick:()=>b.current?.click(),disabled:E,"aria-label":`Browse for a JSON file`,children:`Browse file…`})]}),h&&(0,z.jsxs)(`span`,{className:K.fileError,role:`alert`,children:[`⚠️ `,h]}),(0,z.jsx)(`label`,{htmlFor:`mock-upload-textarea`,className:K.textareaLabel,children:`JSON Payload`}),(0,z.jsx)(`textarea`,{id:`mock-upload-textarea`,ref:y,className:K.textarea,value:l,onChange:e=>{u(e.target.value),g(null)},onKeyDown:ne,placeholder:`Paste JSON here, or drop / browse a .json file above`,rows:10,spellCheck:!1,"aria-describedby":se?`mock-upload-validation`:void 0}),se&&(0,z.jsx)(`span`,{id:`mock-upload-validation`,className:K.validationError,role:`status`,children:`⚠️ Invalid JSON — check syntax`}),(0,z.jsx)(`span`,{className:K.keyboardHint,children:ks?`⌘+Enter to ${c.toLowerCase()}`:`Ctrl+Enter to ${c.toLowerCase()}`}),(0,z.jsxs)(`div`,{className:K.headers,children:[(0,z.jsx)(`span`,{className:K.textareaLabel,children:`Headers (optional)`}),(0,z.jsxs)(`div`,{className:K.rows,role:`group`,"aria-label":`Mock headers`,children:[d.map(e=>{let t=C.get(e.key);return(0,z.jsxs)(`div`,{className:K.rowBlock,children:[(0,z.jsxs)(`div`,{className:K.row,children:[(0,z.jsx)(`input`,{className:`${K.input} ${K.rowName}${t?` ${K.inputInvalid}`:``}`,type:`text`,value:e.name,onChange:t=>j(e.key,{name:t.target.value}),placeholder:`name`,"aria-label":`Header name`,autoComplete:`off`,spellCheck:!1,disabled:E}),(0,z.jsx)(`input`,{className:`${K.input} ${K.rowValue}`,type:`text`,value:e.value,onChange:t=>j(e.key,{value:t.target.value}),placeholder:`value`,"aria-label":`Value of header ${e.name||`(unnamed)`}`,autoComplete:`off`,disabled:E}),(0,z.jsx)(`button`,{type:`button`,className:K.entryRemove,onClick:()=>M(e.key),disabled:E,"aria-label":`Remove header ${e.name||`(unnamed)`}`,title:`Remove header`,children:`×`})]}),t&&(0,z.jsxs)(`span`,{className:K.entryIssue,role:`alert`,children:[`⚠️ `,t]})]},e.key)}),(0,z.jsx)(`button`,{type:`button`,className:K.linkButton,onClick:re,disabled:E,children:`+ Add header`})]}),(0,z.jsx)(`span`,{className:K.inputHintsNote,children:`The headers become the instance's input.header, read case-insensitively by the graph, as a real request's are.`})]}),p&&(0,z.jsxs)(`div`,{className:K.errorBanner,role:`alert`,children:[`❌ Upload failed: `,p]})]}),(0,z.jsxs)(`footer`,{className:K.footer,children:[(0,z.jsx)(`button`,{className:K.formatButton,onClick:N,disabled:!x||E,title:`Format JSON`,"aria-label":`Format JSON`,children:`Format`}),(0,z.jsxs)(`div`,{className:K.footerActions,children:[(0,z.jsx)(`button`,{className:K.cancelButton,onClick:k,disabled:E,children:`Cancel`}),(0,z.jsx)(`button`,{className:K.uploadButton,onClick:te,disabled:!T||E,"aria-busy":E,children:E?(0,z.jsxs)(z.Fragment,{children:[(0,z.jsx)(`span`,{className:K.spinner,"aria-hidden":`true`}),` Uploading…`]}):c===`Upload`?`Upload ▶`:c})]})]})]})})}var q={root:`_root_iz4rh_7`,card:`_card_iz4rh_17`,ribbon:`_ribbon_iz4rh_30`,titleGroup:`_titleGroup_iz4rh_41`,title:`_title_iz4rh_41`,subtitle:`_subtitle_iz4rh_54`,ribbonClose:`_ribbonClose_iz4rh_61`,ribbonCloseIcon:`_ribbonCloseIcon_iz4rh_90`,body:`_body_iz4rh_96`,description:`_description_iz4rh_105`,empty:`_empty_iz4rh_106`,hint:`_hint_iz4rh_115`,sectionTitle:`_sectionTitle_iz4rh_123`,dropZone:`_dropZone_iz4rh_133`,dropZoneActive:`_dropZoneActive_iz4rh_153`,dropZoneIcon:`_dropZoneIcon_iz4rh_159`,dropZoneText:`_dropZoneText_iz4rh_165`,dropZoneOr:`_dropZoneOr_iz4rh_178`,browseButton:`_browseButton_iz4rh_184`,fileInputHidden:`_fileInputHidden_iz4rh_212`,addRow:`_addRow_iz4rh_216`,fileErrors:`_fileErrors_iz4rh_223`,entryList:`_entryList_iz4rh_235`,entry:`_entry_iz4rh_235`,entryFlagged:`_entryFlagged_iz4rh_255`,entryMain:`_entryMain_iz4rh_260`,entryId:`_entryId_iz4rh_267`,entryCounts:`_entryCounts_iz4rh_274`,entryIssue:`_entryIssue_iz4rh_280`,entryRemove:`_entryRemove_iz4rh_285`,entryAction:`_entryAction_iz4rh_286`,manifestList:`_manifestList_iz4rh_323`,manifestRow:`_manifestRow_iz4rh_330`,manifestKey:`_manifestKey_iz4rh_337`,manifestValue:`_manifestValue_iz4rh_343`,fieldLabel:`_fieldLabel_iz4rh_349`,inputRow:`_inputRow_iz4rh_355`,input:`_input_iz4rh_355`,inputInvalid:`_inputInvalid_iz4rh_383`,extension:`_extension_iz4rh_387`,hintError:`_hintError_iz4rh_400`,rows:`_rows_iz4rh_404`,rowBlock:`_rowBlock_iz4rh_410`,row:`_row_iz4rh_404`,rowName:`_rowName_iz4rh_422`,rowValue:`_rowValue_iz4rh_426`,linkButton:`_linkButton_iz4rh_430`,errorBanner:`_errorBanner_iz4rh_455`,footer:`_footer_iz4rh_466`,footerNote:`_footerNote_iz4rh_478`,footerActions:`_footerActions_iz4rh_485`,secondaryButton:`_secondaryButton_iz4rh_492`,primaryButton:`_primaryButton_iz4rh_493`,spinner:`_spinner_iz4rh_543`,spin:`_spin_iz4rh_543`},js=`/api/graph-set/pack`,Ms=`/api/graph-set/unpack`,Ns=`.pack`,Ps=`.json`,Fs=`graph_id`,Is=new Set([`format`,`format_version`]),Ls=[`version`,`description`];function Rs(e){let t=e.trim();return t.toLowerCase().endsWith(Ps)?t.slice(0,t.length-5):``}function zs(e){return e.name.toLowerCase().endsWith(Ns)}function Bs(e){return`${e}${Ns}`}function Vs(e){let t=e.trim();return t===``?`A set name is required - it names the set and its file.`:$a(t)?null:`A set name uses ${Qa}.`}function Hs(e){let t=new Map,n=new Map;for(let t of e)n.set(t.id,(n.get(t.id)??0)+1);for(let r of e)$a(r.id)?r.name!==null&&r.name!==r.id?t.set(r.key,`The root node's name "${r.name}" differs from the graph id.`):(n.get(r.id)??0)>1&&t.set(r.key,`Duplicate graph id "${r.id}" - remove one.`):t.set(r.key,`The graph id uses ${Qa}; name the file <graph-id>.json.`);return t}function Us(e,t){let n=new Map,r=new Map;for(let t of e){let e=t.name.trim();e!==``&&r.set(e,(r.get(e)??0)+1)}for(let i of e){let e=i.name.trim(),a=i.value.trim();e===``&&a===``||(e===``?n.set(i.key,`A value needs a field name.`):e===`set`?n.set(i.key,`"set" is written from the set name.`):Is.has(e)?n.set(i.key,`"${e}" is written by the packager.`):(r.get(e)??0)>1?n.set(i.key,`Duplicate field "${e}".`):e===`graph_id`&&a!==``&&!t.has(a)&&n.set(i.key,`"${a}" is not a graph of the set.`))}return n}function Ws(e,t,n){let r={set:e.trim()};for(let e of t){let t=e.name.trim(),n=e.value.trim();t!==``&&n!==``&&(r[t]=n)}let i={};for(let e of n)i[e.id]=e.model;return{manifest:r,graphs:i}}function Gs(e){return typeof e==`object`&&!!e&&!Array.isArray(e)}function Ks(e){if(!Gs(e)||!Gs(e.manifest)||!Gs(e.graphs))throw Error(`The answer is not a graph set: expected 'manifest' and 'graphs'.`);let t={};for(let[n,r]of Object.entries(e.manifest)){if(typeof r!=`string`)throw Error(`Manifest field '${n}' is not text.`);t[n]=r}let n=[];for(let[t,r]of Object.entries(e.graphs)){let e=io(r);if(!e.ok)throw Error(`Graph '${t}': ${e.error}`);n.push({id:t,model:e.model,name:e.name,nodeCount:e.nodeCount,connectionCount:e.connectionCount})}return{manifest:t,graphs:n}}function qs(e,t){return Object.entries(e).filter(([e])=>e!==`set`&&!Is.has(e)).map(([e,n])=>({key:t(),name:e,value:n}))}var Js=`📦 Graph set packaging`;function Ys(e,t){return`${e} ${t}${e===1?``:`s`}`}function Xs({controller:e,supportsFolderPicker:t}){let{mode:n,busy:r,close:i,leaveInspect:a,entries:o,entryIssues:s,addFiles:c,fileErrors:l,canAddCurrentGraph:u,addCurrentGraph:d,removeEntry:f,clearEntries:p,setName:m,setSetName:h,setNameError:g,rows:_,rowIssues:v,updateRow:y,addRow:b,removeRow:x,canPack:S,pack:C,isPacking:w,packError:ee,inspected:T,isReading:E,importInspectedGraph:D,editInspectedSet:O}=e,[k,A]=(0,P.useState)(!1),[te,ne]=(0,P.useState)(!1),re=(0,P.useRef)(null),j=(0,P.useRef)(null),M=(0,P.useId)(),N=(0,P.useId)(),ie=(0,P.useRef)(()=>void 0);ie.current=()=>{r||(n===`inspect`?a():i())},(0,P.useEffect)(()=>{let e=e=>{e.key===`Escape`&&(e.preventDefault(),ie.current())};return document.addEventListener(`keydown`,e),()=>document.removeEventListener(`keydown`,e)},[]),(0,P.useEffect)(()=>{n===`assemble`&&re.current?.focus()},[n]);let F=(0,P.useCallback)(e=>{e.preventDefault(),e.stopPropagation(),k||A(!0)},[k]),ae=(0,P.useCallback)(e=>{e.preventDefault(),e.stopPropagation(),(e.currentTarget===e.target||!e.currentTarget.contains(e.relatedTarget))&&A(!1)},[]),I=(0,P.useCallback)(e=>{e.preventDefault(),e.stopPropagation(),A(!1),!r&&c(e.dataTransfer.files)},[r,c]),L=(0,P.useCallback)(e=>{let t=e.target.files;t&&t.length>0&&c(t),e.target.value=``},[c]),oe=m.trim(),se=te&&g!==null,ce=Bs(oe||`set-name`),le=(0,z.jsxs)(`div`,{className:`${q.dropZone} ${k?q.dropZoneActive:``}`,onDragOver:F,onDragLeave:ae,onDrop:I,"aria-label":`Drop graph files or a graph set here`,children:[(0,z.jsx)(`span`,{className:q.dropZoneIcon,children:`📂`}),(0,z.jsxs)(`span`,{className:q.dropZoneText,children:[`Drop `,(0,z.jsx)(`code`,{children:`.json`}),` graph files here, or a `,(0,z.jsx)(`code`,{children:Ns}),` to inspect`]}),(0,z.jsx)(`span`,{className:q.dropZoneOr,children:`— or —`}),(0,z.jsx)(`input`,{ref:j,type:`file`,multiple:!0,accept:`.json,${Ns},application/json,application/octet-stream`,className:q.fileInputHidden,"aria-hidden":`true`,tabIndex:-1,onChange:L}),(0,z.jsx)(`button`,{type:`button`,className:q.browseButton,onClick:()=>j.current?.click(),disabled:r,"aria-label":`Browse for graph files`,children:`Browse files…`})]});if(n===`inspect`&&T){let{fileName:e,set:t}=T,n=Object.entries(t.manifest);return(0,z.jsx)(`div`,{className:q.root,children:(0,z.jsxs)(`section`,{className:q.card,"aria-label":`📦 Graph set`,children:[(0,z.jsxs)(`header`,{className:q.ribbon,children:[(0,z.jsxs)(`div`,{className:q.titleGroup,children:[(0,z.jsx)(`span`,{className:q.title,children:`📦 Graph set`}),(0,z.jsx)(`span`,{className:q.subtitle,children:e})]}),(0,z.jsx)(`button`,{type:`button`,className:q.ribbonClose,onClick:i,"aria-label":`Close graph-set panel`,title:`Close`,disabled:r,children:(0,z.jsx)(_s,{className:q.ribbonCloseIcon,"aria-hidden":`true`,focusable:`false`})})]}),(0,z.jsxs)(`div`,{className:q.body,children:[(0,z.jsx)(`p`,{className:q.description,children:`What the engine read from the file: its manifest and its graphs. Import a graph as the session's draft to review it, or edit the whole set and pack it again.`}),(0,z.jsx)(`h3`,{className:q.sectionTitle,children:`Manifest`}),(0,z.jsx)(`dl`,{className:q.manifestList,"aria-label":`Manifest`,children:n.map(([e,t])=>(0,z.jsxs)(`div`,{className:q.manifestRow,children:[(0,z.jsx)(`dt`,{className:q.manifestKey,children:e}),(0,z.jsx)(`dd`,{className:q.manifestValue,children:t})]},e))}),(0,z.jsxs)(`h3`,{className:q.sectionTitle,children:[`Graphs (`,t.graphs.length,`)`]}),(0,z.jsx)(`ul`,{className:q.entryList,"aria-label":`Graphs in the set`,children:t.graphs.map(e=>(0,z.jsxs)(`li`,{className:q.entry,children:[(0,z.jsxs)(`div`,{className:q.entryMain,children:[(0,z.jsx)(`code`,{className:q.entryId,children:e.id}),(0,z.jsxs)(`span`,{className:q.entryCounts,children:[Ys(e.nodeCount,`node`),` · `,Ys(e.connectionCount,`connection`)]})]}),(0,z.jsx)(`button`,{type:`button`,className:q.entryAction,onClick:()=>{D(e.id)},title:`Replace the session's draft with this graph (the UI asks first when a graph is loaded)`,children:`Import as draft`})]},e.id))}),le,l.length>0&&(0,z.jsx)(`ul`,{className:q.fileErrors,role:`alert`,children:l.map(e=>(0,z.jsxs)(`li`,{children:[`⚠️ `,e]},e))})]}),(0,z.jsxs)(`footer`,{className:q.footer,children:[(0,z.jsx)(`button`,{type:`button`,className:q.secondaryButton,onClick:a,disabled:r,children:`Back`}),(0,z.jsx)(`div`,{className:q.footerActions,children:(0,z.jsx)(`button`,{type:`button`,className:q.primaryButton,onClick:O,disabled:r,title:`Loads this set's graphs and manifest into the editor, replacing its list`,children:`Edit as new set`})})]})]})})}return(0,z.jsx)(`div`,{className:q.root,children:(0,z.jsxs)(`section`,{className:q.card,"aria-label":Js,children:[(0,z.jsxs)(`header`,{className:q.ribbon,children:[(0,z.jsxs)(`div`,{className:q.titleGroup,children:[(0,z.jsx)(`span`,{className:q.title,children:Js}),(0,z.jsx)(`span`,{className:q.subtitle,children:ce})]}),(0,z.jsx)(`button`,{type:`button`,className:q.ribbonClose,onClick:i,"aria-label":`Close graph-set panel`,title:`Close (Esc)`,disabled:r,children:(0,z.jsx)(_s,{className:q.ribbonCloseIcon,"aria-hidden":`true`,focusable:`false`})})]}),(0,z.jsxs)(`div`,{className:q.body,children:[(0,z.jsxs)(`p`,{className:q.description,children:[`Graph models packed together as one set, `,(0,z.jsx)(`code`,{children:ce}),`, checked by the deployment gate as they are packed. A set deploys all or none through `,(0,z.jsx)(`code`,{children:`graphs.yaml`}),`; a set of one graph is how one graph is signed.`]}),le,(0,z.jsxs)(`div`,{className:q.addRow,children:[(0,z.jsx)(`button`,{type:`button`,className:q.secondaryButton,onClick:d,disabled:!u||r,title:u?`Adds the graph in the Graph view, named after its graph id`:`Load a graph first`,children:`Add current graph`}),o.length>0&&(0,z.jsx)(`button`,{type:`button`,className:q.linkButton,onClick:p,disabled:r,children:`Clear list`})]}),l.length>0&&(0,z.jsx)(`ul`,{className:q.fileErrors,role:`alert`,children:l.map(e=>(0,z.jsxs)(`li`,{children:[`⚠️ `,e]},e))}),(0,z.jsxs)(`h3`,{className:q.sectionTitle,children:[`Graphs (`,o.length,`)`]}),o.length===0?(0,z.jsxs)(`p`,{className:q.empty,children:[`No graphs yet. Drop `,(0,z.jsx)(`code`,{children:`<graph-id>.json`}),` files or add the current graph.`]}):(0,z.jsx)(`ul`,{className:q.entryList,"aria-label":`Graphs in the set`,children:o.map(e=>{let t=s.get(e.key);return(0,z.jsxs)(`li`,{className:`${q.entry}${t?` ${q.entryFlagged}`:``}`,children:[(0,z.jsxs)(`div`,{className:q.entryMain,children:[(0,z.jsx)(`code`,{className:q.entryId,children:e.id||`(no graph id)`}),(0,z.jsxs)(`span`,{className:q.entryCounts,children:[Ys(e.nodeCount,`node`),` · `,Ys(e.connectionCount,`connection`),e.label!==`${e.id}.json`&&(0,z.jsxs)(z.Fragment,{children:[` · `,e.label]})]}),t&&(0,z.jsxs)(`span`,{className:q.entryIssue,role:`alert`,children:[`⚠️ `,t]})]}),(0,z.jsx)(`button`,{type:`button`,className:q.entryRemove,onClick:()=>f(e.key),disabled:r,"aria-label":`Remove ${e.id||e.label} from the set`,title:`Remove from the set`,children:`×`})]},e.key)})}),(0,z.jsx)(`h3`,{className:q.sectionTitle,children:`Manifest`}),(0,z.jsx)(`label`,{className:q.fieldLabel,htmlFor:M,children:`Set name`}),(0,z.jsxs)(`div`,{className:q.inputRow,children:[(0,z.jsx)(`input`,{id:M,ref:re,className:`${q.input}${se?` ${q.inputInvalid}`:``}`,type:`text`,value:m,onChange:e=>{h(e.target.value),ne(!0)},onBlur:()=>ne(!0),"aria-describedby":N,"aria-invalid":se,autoComplete:`off`,spellCheck:!1,maxLength:80,disabled:r}),(0,z.jsx)(`span`,{className:q.extension,"aria-hidden":`true`,children:Ns})]}),(0,z.jsx)(`p`,{id:N,className:`${q.hint}${se?` ${q.hintError}`:``}`,role:se?`alert`:void 0,children:se?g:`Required. It names the set and its file, ${ce}.`}),(0,z.jsx)(`div`,{className:q.fieldLabel,children:`Fields`}),(0,z.jsxs)(`div`,{className:q.rows,role:`group`,"aria-label":`Manifest fields`,children:[_.map(e=>{let t=v.get(e.key);return(0,z.jsxs)(`div`,{className:q.rowBlock,children:[(0,z.jsxs)(`div`,{className:q.row,children:[(0,z.jsx)(`input`,{className:`${q.input} ${q.rowName}${t?` ${q.inputInvalid}`:``}`,type:`text`,value:e.name,onChange:t=>y(e.key,{name:t.target.value}),placeholder:`field`,"aria-label":`Manifest field name`,autoComplete:`off`,spellCheck:!1,disabled:r}),(0,z.jsx)(`input`,{className:`${q.input} ${q.rowValue}`,type:`text`,value:e.value,onChange:t=>y(e.key,{value:t.target.value}),placeholder:`value`,"aria-label":`Value of manifest field ${e.name||`(unnamed)`}`,autoComplete:`off`,disabled:r}),(0,z.jsx)(`button`,{type:`button`,className:q.entryRemove,onClick:()=>x(e.key),disabled:r,"aria-label":`Remove manifest field ${e.name||`(unnamed)`}`,title:`Remove field`,children:`×`})]}),t&&(0,z.jsxs)(`span`,{className:q.entryIssue,role:`alert`,children:[`⚠️ `,t]})]},e.key)}),(0,z.jsx)(`button`,{type:`button`,className:q.linkButton,onClick:b,disabled:r,children:`+ Add field`})]}),(0,z.jsxs)(`p`,{className:q.hint,children:[`Optional text fields that travel in the manifest, such as `,(0,z.jsx)(`code`,{children:`version`}),` and `,(0,z.jsx)(`code`,{children:`description`}),`; a blank value is left out. `,(0,z.jsx)(`code`,{children:Fs}),` names the set's entry-point graph.`]}),ee&&(0,z.jsxs)(`div`,{className:q.errorBanner,role:`alert`,children:[`❌ `,ee]})]}),(0,z.jsxs)(`footer`,{className:q.footer,children:[(0,z.jsx)(`span`,{className:q.footerNote,children:t?`The next dialog picks the folder.`:`The browser saves it to its download folder.`}),(0,z.jsxs)(`div`,{className:q.footerActions,children:[(0,z.jsx)(`button`,{type:`button`,className:q.secondaryButton,onClick:i,disabled:r,children:`Cancel`}),(0,z.jsx)(`button`,{type:`button`,className:q.primaryButton,onClick:()=>{ne(!0),C()},disabled:!S,"aria-busy":w,title:S?`Packs the set on the engine and saves ${ce}`:`Add a graph and name the set first`,children:w?(0,z.jsxs)(z.Fragment,{children:[(0,z.jsx)(`span`,{className:q.spinner,"aria-hidden":`true`}),` Packing…`]}):E?(0,z.jsxs)(z.Fragment,{children:[(0,z.jsx)(`span`,{className:q.spinner,"aria-hidden":`true`}),` Reading…`]}):`Pack and download`})]})]})]})})}var J={root:`_root_7s9c5_8`,card:`_card_7s9c5_18`,ribbon:`_ribbon_7s9c5_31`,titleGroup:`_titleGroup_7s9c5_42`,title:`_title_7s9c5_42`,subtitle:`_subtitle_7s9c5_55`,ribbonClose:`_ribbonClose_7s9c5_62`,ribbonCloseIcon:`_ribbonCloseIcon_7s9c5_91`,tabs:`_tabs_7s9c5_97`,tab:`_tab_7s9c5_97`,tabActive:`_tabActive_7s9c5_131`,tabNode:`_tabNode_7s9c5_138`,dirtyDot:`_dirtyDot_7s9c5_144`,body:`_body_7s9c5_153`,description:`_description_7s9c5_162`,empty:`_empty_7s9c5_163`,hint:`_hint_7s9c5_172`,sectionTitle:`_sectionTitle_7s9c5_173`,issueList:`_issueList_7s9c5_174`,sectionHeader:`_sectionHeader_7s9c5_182`,loading:`_loading_7s9c5_225`,rows:`_rows_7s9c5_235`,rowCard:`_rowCard_7s9c5_241`,rowFlagged:`_rowFlagged_7s9c5_251`,rowLine:`_rowLine_7s9c5_256`,input:`_input_7s9c5_263`,select:`_select_7s9c5_264`,inputInvalid:`_inputInvalid_7s9c5_287`,pathInput:`_pathInput_7s9c5_291`,typeSelect:`_typeSelect_7s9c5_296`,itemSelect:`_itemSelect_7s9c5_301`,descriptionInput:`_descriptionInput_7s9c5_306`,exampleInput:`_exampleInput_7s9c5_311`,requiredLabel:`_requiredLabel_7s9c5_315`,chips:`_chips_7s9c5_331`,chip:`_chip_7s9c5_331`,chipDeclared:`_chipDeclared_7s9c5_353`,chipDiscovered:`_chipDiscovered_7s9c5_358`,chipRun:`_chipRun_7s9c5_362`,chipNew:`_chipNew_7s9c5_367`,chipConstraints:`_chipConstraints_7s9c5_371`,rowIssue:`_rowIssue_7s9c5_378`,removeButton:`_removeButton_7s9c5_383`,linkButton:`_linkButton_7s9c5_412`,errorBanner:`_errorBanner_7s9c5_437`,warningBanner:`_warningBanner_7s9c5_452`,footer:`_footer_7s9c5_462`,footerTools:`_footerTools_7s9c5_473`,footerActions:`_footerActions_7s9c5_480`,secondaryButton:`_secondaryButton_7s9c5_487`,primaryButton:`_primaryButton_7s9c5_488`,spinner:`_spinner_7s9c5_538`,spin:`_spin_7s9c5_538`,spinnerDark:`_spinnerDark_7s9c5_549`};async function Zs(e){let t=await e.text();try{let e=JSON.parse(t);if(typeof e.message==`string`)return e.message}catch{}return t||`HTTP ${e.status}`}var Qs=[``,`string`,`number`,`integer`,`boolean`,`object`,`array`],$s=[``,`string`,`number`,`integer`,`boolean`],ec=[``,`string`,`number`,`integer`,`boolean`],tc=new Set([`type`,`properties`,`required`,`items`,`description`,`example`]),nc={input:`root`,output:`end`},rc=`schema`;function ic(e){return typeof e==`object`&&!!e&&!Array.isArray(e)}function ac(e,t){if(!ic(e)||!Array.isArray(e.paths))throw Error(`The contract view has no '${t}' part.`);let n=[];for(let t of e.paths){if(!ic(t)||typeof t.path!=`string`)continue;let e=t.origin===`declared`||t.origin===`both`?t.origin:`discovered`;n.push({path:t.path,type:typeof t.type==`string`?t.type:void 0,origin:e,required:t.required===!0,usedBy:Array.isArray(t.usedBy)?t.usedBy.filter(e=>typeof e==`string`):[]})}return{declared:e.declared===!0,schema:ic(e.schema)?e.schema:void 0,paths:n}}function oc(e){if(!ic(e)||typeof e.graph!=`string`||!ic(e.input)||!ic(e.output))throw Error(`The answer is not a graph contract: expected 'graph', 'input' and 'output'.`);let t=Array.isArray(e.output.status)?e.output.status.filter(e=>typeof e==`number`):[];return{graph:e.graph,purpose:typeof e.purpose==`string`?e.purpose:void 0,input:{body:ac(e.input.body,`input.body`),header:ac(e.input.header,`input.header`)},output:{body:ac(e.output.body,`output.body`),header:ac(e.output.header,`output.header`),status:t},issues:Array.isArray(e.issues)?e.issues.filter(e=>typeof e==`string`):[]}}var sc=/^([A-Za-z0-9_-]+)(\[\])?$/,cc=/^[A-Za-z0-9_-]+$/,lc=`dot-separated names of letters, digits, hyphen or underscore, with [] after an array`,uc=`letters, digits, hyphen or underscore`;function dc(e){let t=e.trim();if(t===``)return null;let n=[];for(let e of t.split(`.`)){let t=e.match(sc);if(!t)return null;n.push({name:t[1],array:t[2]!==void 0})}return n}function fc(e){return cc.test(e.trim())}function pc(e,t){let n=`${t}.`;return(e.startsWith(n)?e.slice(n.length):e).replace(/\[(\*|\d+)\]/g,`[]`)}function mc(e){let t=e.lastIndexOf(`.`);return t<0?null:e.slice(0,t)}function hc(e,t){let n=dc(t);if(!e||!n)return;let r=e;for(let e of n){let t=r?.properties,n=ic(t)?t[e.name]:void 0;if(!ic(n))return;r=n,e.array&&(r=ic(r.items)?r.items:{})}return r}function gc(e,t){let n=mc(t),r=n===null?e:hc(e,n),i=dc(t);if(!r||!i)return!1;let a=i[i.length-1].name;return Array.isArray(r.required)&&r.required.includes(a)}function _c(e,t){return typeof e==`string`&&t.includes(e)?e:``}function vc(e){let t=e?.items,n=ic(t)?t.type:void 0;return typeof n==`string`&&ec.includes(n)?n:``}function yc(e){return e==null?``:typeof e==`string`?e:typeof e==`number`||typeof e==`boolean`?String(e):JSON.stringify(e)}function bc(e){let t={};if(!e)return t;for(let[n,r]of Object.entries(e))tc.has(n)||(t[n]=r);return t}function xc(e,t,n,r,i){let a=n===`header`?$s:Qs,o=[],s=new Set;for(let n of e.paths){let c=pc(n.path,r);if(s.has(c))continue;s.add(c);let l=c.endsWith(`[]`)?c.slice(0,-2):c,u=hc(t,l),d=hc(e.schema,l),f=_c(n.type??d?.type,a);o.push({key:i(),path:c,type:c.endsWith(`[]`)?`array`:f,itemType:c.endsWith(`[]`)?vc(u)||vc(d):``,required:n.required===!0||gc(t,c),description:yc(u?.description),example:yc(u?.example),source:n.origin,usedBy:n.usedBy??[],constraints:bc(u)})}return o}var Sc=80;function Cc(e){return typeof e==`string`?`string`:typeof e==`boolean`?`boolean`:typeof e==`number`?Number.isInteger(e)?`integer`:`number`:``}function wc(e){if(e==null||typeof e==`object`)return``;let t=String(e);return t.length>Sc?`${t.slice(0,Sc-1)}…`:t}function Tc(e,t,n,r,i){if(ic(t))for(let[a,o]of Object.entries(t)){if(!cc.test(a))continue;let t=e?`${e}.${a}`:a;if(i===`header`){n.push(Ec(r(),t,`string`,``,wc(o)));continue}if(Array.isArray(o)){let e=o.find(e=>e!=null),a=ic(e)?``:Cc(e);if(n.push(Ec(r(),`${t}[]`,`array`,a,``)),ic(e)){let e={};for(let t of o)if(ic(t))for(let[n,r]of Object.entries(t))n in e||(e[n]=r);Tc(`${t}[]`,e,n,r,i)}}else ic(o)?(n.push(Ec(r(),t,`object`,``,``)),Tc(t,o,n,r,i)):n.push(Ec(r(),t,Cc(o),``,wc(o)))}}function Ec(e,t,n,r,i){return{key:e,path:t,type:n,itemType:r,required:!1,description:``,example:i,source:`run`,usedBy:[],constraints:{}}}function Dc(e,t,n){let r=[];return Tc(``,e,r,n,t),r}function Oc(e,t){let n=0,r=new Map(e.map(e=>[e.path,e])),i=e.map(e=>({...e}));for(let e of t){let t=r.get(e.path);if(!t){i.push(e),n++;continue}let a=i.find(e=>e.key===t.key),o=!1,s=!1;a.type===``&&e.type!==``&&(a.type=e.type,o=!0),a.type===`array`&&a.itemType===``&&e.itemType!==``&&(a.itemType=e.itemType,o=!0),a.example===``&&e.example!==``&&(a.example=e.example,s=!0),(o||s)&&n++,o&&a.source===`discovered`&&(a.source=`run`)}return{rows:i,changed:n}}function kc(e,t){let n=new Map,r=new Map;for(let t of e){let e=t.path.trim();e!==``&&r.set(e,(r.get(e)??0)+1)}for(let i of e){let a=i.path.trim();if(a===``){n.set(i.key,t===`header`?`A header name is required.`:`A path is required.`);continue}if(t===`header`){fc(a)?$s.includes(i.type)?(r.get(a)??0)>1&&n.set(i.key,`Duplicate header "${a}".`):n.set(i.key,`A header is text: string, number, integer or boolean.`):n.set(i.key,`A header name uses ${uc}.`);continue}dc(a)?(r.get(a)??0)>1?n.set(i.key,`Duplicate path "${a}".`):a.endsWith(`[]`)&&i.type!==`array`?n.set(i.key,`A path ending in [] is an array.`):!a.endsWith(`[]`)&&i.type===`array`?n.set(i.key,`An array path ends in [] - items[] for a list of items.`):i.type!==``&&i.type!==`object`&&i.type!==`array`&&e.some(e=>e!==i&&e.path.trim().startsWith(`${a}.`))?n.set(i.key,`"${a}" has nested paths, so its type is object (or leave it blank).`):i.type===`array`&&i.itemType!==``&&e.some(e=>e!==i&&e.path.trim().startsWith(`${a}.`))&&n.set(i.key,`"${a}" has nested paths, so its items are objects - clear the item type.`):n.set(i.key,`A path uses ${lc}.`)}return n}function Ac(e){return{row:null,array:e,children:new Map}}function jc(e,t){let n=e.row,r={},i={},a=[];for(let[n,r]of e.children)i[n]=jc(r,t),r.row?.required&&a.push(n);let o=e.children.size>0;if(e.array)if(r.type=`array`,o){let e={type:`object`,properties:i};a.length>0&&(e.required=a),r.items=e}else n&&n.itemType!==``?r.items={type:n.itemType}:n&&ic(n.constraints.items)&&(r.items=n.constraints.items);else{let e=n?.type??``;o||e===`object`?(r.type=`object`,o&&(r.properties=i),a.length>0&&(r.required=a)):e!==``&&(r.type=e)}if(n){n.description.trim()!==``&&(r.description=n.description.trim()),n.example.trim()!==``&&(r.example=n.example.trim());for(let[e,t]of Object.entries(n.constraints))e===`items`&&`items`in r||e in r||(r[e]=t)}return r}function Mc(e,t){let n=e.filter(e=>e.path.trim()!==``);if(n.length===0)return null;let r=Ac(!1);for(let e of n){let n=t===`header`?[{name:e.path.trim(),array:!1}]:dc(e.path.trim());if(!n)continue;let i=r;for(let e of n){let t=i.children.get(e.name);t?e.array&&(t.array=!0):(t=Ac(e.array),i.children.set(e.name,t)),i=t}i.row=e}return jc(r,t)}function Nc(e,t){let n={};return e&&(n.body=e),t&&(n.header=t),Object.keys(n).length>0?n:null}var Pc=`'''`;function Fc(e,t,n){if(t!=null){if(Array.isArray(t)){for(let r of t){if(r==null||typeof r==`object`)throw Error(`'${e}' holds a list of objects, which the node grammar cannot carry.`);n.push(er(`${e}[]`,Ic(e,r)))}return}if(ic(t)){for(let[r,i]of Object.entries(t)){if(!cc.test(r))throw Error(`'${e}.${r}' - a property name uses ${uc}.`);Fc(`${e}.${r}`,i,n)}return}n.push(er(e,Ic(e,t)))}}function Ic(e,t){let n=typeof t==`string`?t:String(t);if(n.includes(Pc))throw Error(`'${e}' contains ''' which the node grammar cannot carry.`);return n}function Lc(e,t=rc){let n=[];return Fc(t,e,n),n}function Rc(e){let t=e.trim();return t===`schema`||t.startsWith(`schema.`)||t.startsWith(`schema[`)}function zc(e){let t=e?.properties[rc];return ic(t)?t:void 0}function Bc(e){return Object.entries(e).map(([e,t])=>{if(Array.isArray(t))return`${e} (${t.length})`;if(ic(t))return e;let n=String(t);return n.length>16?e:`${e} ${n}`}).join(` · `)}function Vc(e,t){let n=ur(e);if(!n.valid||!n.formState)throw Error(n.message??`The node cannot be edited from the Schema panel.`);let r=n.formState.properties.filter(e=>e.key.trim()!==``&&!Rc(e.key)),i=t?Lc(t):[];return hr({...n.formState,properties:[...r,...i]},e.alias)}function Hc(e){return`/api/openapi/session/${e}?view=contract`}function Uc(e){return`/api/openapi/session/${e}`}function Wc(e,t){return`/api/inspect/${e}/${t}`}var Gc=1e4,Kc=[`input.body`,`input.header`,`output.body`,`output.header`];function qc(e,t){return`${e}.${t}`}var Jc={"input.body":[],"input.header":[],"output.body":[],"output.header":[]},Yc={input:!1,output:!1};function Xc(e,t){let n=(e.headers.get(`Content-Disposition`)??``).match(/filename="?([^";]+)"?/);return n?n[1]:t}function Zc({addToast:e,sessionId:t,connected:n,graphData:r,graphName:i,bus:a,sendRawText:o}){let s=(0,P.useRef)(1),c=(0,P.useCallback)(()=>s.current++,[]),[l,u]=(0,P.useState)(!1),[d,f]=(0,P.useState)(`input`),[p,m]=(0,P.useState)(null),[h,g]=(0,P.useState)(!1),[_,v]=(0,P.useState)(null),[y,b]=(0,P.useState)(Jc),[x,S]=(0,P.useState)(Yc),[C,w]=(0,P.useState)(!1),[ee,T]=(0,P.useState)(!1),[E,D]=(0,P.useState)(null),[O,k]=(0,P.useState)(!1),A=(0,P.useRef)(r);A.current=r;let te=(0,P.useRef)(y);te.current=y;let ne=(0,P.useRef)(x);ne.current=x;let re=(0,P.useRef)(l);re.current=l;let j=(0,P.useRef)(null),M=(0,P.useRef)(null),N=(0,P.useCallback)(e=>r?.nodes.find(t=>t.alias===nc[e])??null,[r]),ie=(0,P.useCallback)(()=>{j.current&&=(clearTimeout(j.current.timer),null),T(!1)},[]),F=(0,P.useCallback)(async(e=!1)=>{if(t===null){v(`The session id is not known yet.`);return}M.current?.abort();let n=new AbortController;M.current=n,g(!0),v(null);try{let r=await fetch(Hc(t),{headers:{Accept:`application/json`},signal:n.signal});if(!r.ok){v(await Zs(r));return}let i=oc(await r.json());if(!e&&(ne.current.input||ne.current.output))return;let a=A.current,o=zc(a?.nodes.find(e=>e.alias===nc.input)),s=zc(a?.nodes.find(e=>e.alias===nc.output)),l=(e,t)=>e&&typeof e[t]==`object`&&e[t]!==null?e[t]:void 0;m(i),b({"input.body":xc(i.input.body,l(o,`body`),`body`,`input.body`,c),"input.header":xc(i.input.header,l(o,`header`),`header`,`input.header`,c),"output.body":xc(i.output.body,l(s,`body`),`body`,`output.body`,c),"output.header":xc(i.output.header,l(s,`header`),`header`,`output.header`,c)}),S(Yc),D(null)}catch(e){if(e.name===`AbortError`)return;v(`The contract could not be read: ${e.message}`)}finally{M.current===n&&(M.current=null),g(!1)}},[t,c]),ae=(0,P.useRef)(F);ae.current=F,(0,P.useEffect)(()=>()=>{M.current?.abort()},[]),(0,P.useEffect)(()=>{m(null),b(Jc),S(Yc),v(null),ie()},[t,ie]);let I=(0,P.useCallback)((e=`input`)=>{f(e),u(!0);let t=ne.current;!t.input&&!t.output&&ae.current()},[]),L=(0,P.useCallback)(()=>{M.current?.abort(),M.current=null,g(!1),u(!1)},[]),oe=(0,P.useCallback)(()=>{ae.current(!0)},[]);(0,P.useEffect)(()=>a.on(`graph.mutation`,()=>{if(!re.current)return;let e=ne.current;e.input||e.output||j.current||ae.current()}),[a]);let se=(0,P.useCallback)(e=>{let t=e.startsWith(`input`)?`input`:`output`;S(e=>e[t]?e:{...e,[t]:!0}),D(null)},[]),ce=(0,P.useCallback)((e,t,n)=>{b(r=>({...r,[e]:r[e].map(e=>e.key===t?{...e,...n}:e)})),se(e)},[se]),le=(0,P.useCallback)(e=>{let t=e.endsWith(`header`)?`string`:``;b(n=>({...n,[e]:[...n[e],{key:c(),path:``,type:t,itemType:``,required:!1,description:``,example:``,source:`new`,usedBy:[],constraints:{}}]})),se(e)},[c,se]),ue=(0,P.useCallback)((e,t)=>{b(n=>({...n,[e]:n[e].filter(e=>e.key!==t)})),se(e)},[se]),R=(0,P.useMemo)(()=>({"input.body":kc(y[`input.body`],`body`),"input.header":kc(y[`input.header`],`header`),"output.body":kc(y[`output.body`],`body`),"output.header":kc(y[`output.header`],`header`)}),[y]),de=(0,P.useCallback)(e=>(p?.issues??[]).filter(t=>t.startsWith(`${e}.`)),[p]),z=(0,P.useCallback)(async()=>{if(t===null){e(`The session id is not known yet.`,`error`);return}w(!0);try{let n=await Promise.all(Kc.map(async e=>{let n=await fetch(Wc(t,e),{headers:{Accept:`application/json`}});if(n.status===404)return null;if(!n.ok)throw Error(await Zs(n));return(await n.json()).outcome??null}));if(n.every(e=>e===null)){e(`No instance data yet - Instantiate, Upload and Run first.`,`info`);return}let r=0,i={input:!1,output:!1},a={...te.current};Kc.forEach((e,t)=>{let o=n[t];if(o===null)return;let s=e.endsWith(`header`)?`header`:`body`,l=Oc(a[e],Dc(o,s,c));l.changed>0&&(a[e]=l.rows,r+=l.changed,i[e.startsWith(`input`)?`input`:`output`]=!0)}),r===0?e(`The last run adds nothing: every row already has its type and example.`,`info`):(b(a),S(e=>({input:e.input||i.input,output:e.output||i.output})),D(null),e(`${r} row${r===1?``:`s`} filled from the last run - review and Save.`,`success`))}catch(t){e(`Fill from last run failed: ${t.message}`,`error`)}finally{w(!1)}},[t,e,c]),fe=R[qc(d,`body`)].size+R[qc(d,`header`)].size,pe=n&&x[d]&&fe===0&&!ee&&!h&&N(d)!==null,me=(0,P.useCallback)(()=>{if(!pe)return;let e=N(d);if(!e)return;let t;try{t=Vc(e,Nc(Mc(y[qc(d,`body`)],`body`),Mc(y[qc(d,`header`)],`header`)))}catch(e){D(e.message);return}if(D(null),!o(t)){D(`The command was not sent because the WebSocket is not open. Your rows are kept.`);return}T(!0),j.current={side:d,alias:e.alias,timer:setTimeout(()=>{j.current=null,T(!1),D(`The engine did not answer within ${Gc/1e3} s - check the console.`)},Gc)}},[pe,N,d,y,o]);(0,P.useEffect)(()=>a.on(`minigraph.nodeAction.textResult`,t=>{let n=j.current;if(!n)return;let r=t.action===`edit-node`&&t.alias===n.alias,i=t.status===`error`&&t.action===null;!r&&!i||(ie(),t.status===`accepted`?(S(e=>({...e,[n.side]:!1})),e(`Schema of node ${n.alias} saved`,`success`),ae.current()):D(t.message))}),[a,e,ie]),(0,P.useEffect)(()=>()=>{j.current&&clearTimeout(j.current.timer)},[]);let he=(0,P.useCallback)(async()=>{if(t===null){e(`The session id is not known yet.`,`error`);return}k(!0);try{let n=await fetch(Uc(t),{headers:{Accept:`application/yaml`}});if(!n.ok){e(`The document could not be read - ${await Zs(n)}`,`error`);return}let r=await uo(await n.text(),Xc(n,`${eo(i)}.yaml`));r.saved&&e(`OpenAPI document saved as ${r.fileName}`,`success`)}catch(t){e(`Download failed: ${t.message}`,`error`)}finally{k(!1)}},[t,i,e]);return{isOpen:l,open:I,close:L,side:d,setSide:f,busy:ee||C||O,contract:p,isLoading:h,loadError:_,reload:oe,rows:y,issues:R,dirty:x,updateRow:ce,addRow:le,removeRow:ue,nodeOf:N,issuesOf:de,fillFromRun:z,isFilling:C,save:me,canSave:pe,isSaving:ee,saveError:E,download:he,isDownloading:O}}var Qc=`📐 Graph schema`,$c={input:`Input`,output:`Output`},el={input:`request`,output:`response`},tl={declared:`declared`,discovered:`discovered`,both:`declared`,run:`from last run`,new:`new`},nl={declared:J.chipDeclared,discovered:J.chipDiscovered,both:J.chipDeclared,run:J.chipRun,new:J.chipNew};function rl(e){let t=e.usedBy.length>0?`Referenced by ${e.usedBy.join(`, `)}`:`Not referenced by the model`;switch(e.source){case`declared`:return`Declared on the node. ${t}.`;case`both`:return`Declared on the node and discovered in the model. ${t}.`;case`discovered`:return`Discovered in the model, not declared yet. ${t}.`;case`run`:return`Typed from the instance's last run.`;default:return`Added here; saved with the declaration.`}}function il(e){return e===``?`(any)`:e}function al({controller:e,graphName:t,supportsFolderPicker:n}){let{side:r,setSide:i,busy:a,close:o,contract:s,isLoading:c,loadError:l,reload:u,rows:d,issues:f,dirty:p,updateRow:m,addRow:h,removeRow:g,nodeOf:_,issuesOf:v,fillFromRun:y,isFilling:b,save:x,canSave:S,isSaving:C,saveError:w,download:ee,isDownloading:T}=e,E=(0,P.useRef)(()=>void 0);E.current=()=>{a||o()},(0,P.useEffect)(()=>{let e=e=>{e.key===`Escape`&&(e.preventDefault(),E.current())};return document.addEventListener(`keydown`,e),()=>document.removeEventListener(`keydown`,e)},[]);let D=_(r),O=nc[r],k=v(r),A=t?.trim()||s?.graph||`draft`,te=e=>{let t=qc(r,e),n=d[t],i=f[t],o=e===`header`?$s:Qs,s=`${r}.${e}`,c=e===`header`?`header`:`path`;return(0,z.jsxs)(z.Fragment,{children:[(0,z.jsxs)(`div`,{className:J.sectionHeader,children:[(0,z.jsxs)(`h3`,{className:J.sectionTitle,children:[e===`header`?`Headers`:`Body`,` `,(0,z.jsx)(`code`,{children:s})]}),(0,z.jsxs)(`span`,{className:J.hint,children:[n.length,` `,c,n.length===1?``:`s`]})]}),n.length===0?(0,z.jsx)(`p`,{className:J.empty,children:e===`header`?(0,z.jsxs)(z.Fragment,{children:[`No header. Add one when the `,el[r],` carries a header the graph `,r===`input`?`reads`:`sets`,`.`]}):(0,z.jsx)(z.Fragment,{children:`No path. Add one, or Fill from last run after a dry run.`})}):(0,z.jsx)(`div`,{className:J.rows,role:`group`,"aria-label":`${s} rows`,children:n.map(n=>{let s=i.get(n.key),l=n.path.trim()||`(new ${c})`,u=Bc(n.constraints);return(0,z.jsxs)(`div`,{className:`${J.rowCard}${s?` ${J.rowFlagged}`:``}`,children:[(0,z.jsxs)(`div`,{className:J.rowLine,children:[(0,z.jsx)(`input`,{className:`${J.input} ${J.pathInput}${s?` ${J.inputInvalid}`:``}`,type:`text`,value:n.path,onChange:e=>m(t,n.key,{path:e.target.value}),placeholder:e===`header`?`X-Header-Name`:`path, items[] for a list`,"aria-label":e===`header`?`Header name`:`Path`,"aria-invalid":!!s,autoComplete:`off`,spellCheck:!1,disabled:a}),(0,z.jsx)(`select`,{className:`${J.select} ${J.typeSelect}`,value:n.type,onChange:e=>m(t,n.key,{type:e.target.value}),"aria-label":`Type of ${l}`,disabled:a,children:o.map(e=>(0,z.jsx)(`option`,{value:e,children:il(e)},e))}),n.type===`array`&&(0,z.jsx)(`select`,{className:`${J.select} ${J.itemSelect}`,value:n.itemType,onChange:e=>m(t,n.key,{itemType:e.target.value}),"aria-label":`Item type of ${l}`,title:`The element type of the list; leave it blank for objects described by nested paths`,disabled:a,children:ec.map(e=>(0,z.jsx)(`option`,{value:e,children:e===``?`of …`:`of ${e}`},e))}),(0,z.jsxs)(`label`,{className:J.requiredLabel,title:r===`input`?`A missing value fails validation`:`Always present in the response`,children:[(0,z.jsx)(`input`,{type:`checkbox`,checked:n.required,onChange:e=>m(t,n.key,{required:e.target.checked}),"aria-label":`Required ${l}`,disabled:a}),`req`]}),(0,z.jsx)(`button`,{type:`button`,className:J.removeButton,onClick:()=>g(t,n.key),disabled:a,"aria-label":`Remove ${l}`,title:`Remove from the declaration`,children:`×`})]}),(0,z.jsxs)(`div`,{className:J.rowLine,children:[(0,z.jsx)(`input`,{className:`${J.input} ${J.descriptionInput}`,type:`text`,value:n.description,onChange:e=>m(t,n.key,{description:e.target.value}),placeholder:`description`,"aria-label":`Description of ${l}`,autoComplete:`off`,disabled:a}),(0,z.jsx)(`input`,{className:`${J.input} ${J.exampleInput}`,type:`text`,value:n.example,onChange:e=>m(t,n.key,{example:e.target.value}),placeholder:`example`,"aria-label":`Example of ${l}`,autoComplete:`off`,spellCheck:!1,disabled:a})]}),(0,z.jsxs)(`div`,{className:J.chips,children:[(0,z.jsx)(`span`,{className:`${J.chip} ${nl[n.source]}`,title:rl(n),children:tl[n.source]}),n.usedBy.length>0&&(0,z.jsxs)(`span`,{className:J.chip,title:`Referenced by ${n.usedBy.join(`, `)}`,children:[`used by `,n.usedBy.length===1?n.usedBy[0]:`${n.usedBy.length} nodes`]}),u!==``&&(0,z.jsx)(`span`,{className:`${J.chip} ${J.chipConstraints}`,title:`Carried unchanged - edit in the node editor:\n${JSON.stringify(n.constraints,null,1)}`,children:u}),s&&(0,z.jsxs)(`span`,{className:J.rowIssue,role:`alert`,children:[`⚠️ `,s]})]})]},n.key)})}),(0,z.jsxs)(`button`,{type:`button`,className:J.linkButton,onClick:()=>h(t),disabled:a,children:[`+ Add `,c]})]})};return(0,z.jsx)(`div`,{className:J.root,children:(0,z.jsxs)(`section`,{className:J.card,"aria-label":Qc,children:[(0,z.jsxs)(`header`,{className:J.ribbon,children:[(0,z.jsxs)(`div`,{className:J.titleGroup,children:[(0,z.jsx)(`span`,{className:J.title,children:Qc}),(0,z.jsxs)(`span`,{className:J.subtitle,children:[A,` · node `,O]})]}),(0,z.jsx)(`button`,{type:`button`,className:J.ribbonClose,onClick:o,"aria-label":`Close schema panel`,title:`Close (Esc)`,disabled:a,children:(0,z.jsx)(_s,{className:J.ribbonCloseIcon,"aria-hidden":`true`,focusable:`false`})})]}),(0,z.jsx)(`div`,{className:J.tabs,role:`tablist`,"aria-label":`Contract side`,children:[`input`,`output`].map(e=>(0,z.jsxs)(`button`,{type:`button`,role:`tab`,className:`${J.tab}${r===e?` ${J.tabActive}`:``}`,"aria-selected":r===e,onClick:()=>i(e),title:`The ${el[e]}: schema on node ${nc[e]}`,children:[$c[e],` `,(0,z.jsx)(`span`,{className:J.tabNode,children:nc[e]}),p[e]&&(0,z.jsx)(`span`,{className:J.dirtyDot,"aria-label":`unsaved`,title:`Unsaved changes`})]},e))}),(0,z.jsxs)(`div`,{className:J.body,role:`tabpanel`,"aria-label":`${$c[r]} contract`,children:[(0,z.jsx)(`p`,{className:J.description,children:r===`input`?(0,z.jsxs)(z.Fragment,{children:[`The request contract, `,(0,z.jsx)(`code`,{children:`schema`}),` on the root node: `,(0,z.jsx)(`code`,{children:`input.body`}),` is validated against it on every run, before anything else - a bad request never reaches the graph. Headers are matched by name, case-insensitively.`]}):(0,z.jsxs)(z.Fragment,{children:[`The response contract, `,(0,z.jsx)(`code`,{children:`schema`}),` on the end node: documentary - it describes `,(0,z.jsx)(`code`,{children:`output.body`}),` and `,(0,z.jsx)(`code`,{children:`output.header`}),` in the OpenAPI document and is never enforced.`]})}),l&&(0,z.jsxs)(`div`,{className:J.errorBanner,role:`alert`,children:[(0,z.jsxs)(`span`,{children:[`❌ `,l]}),(0,z.jsx)(`button`,{type:`button`,className:J.linkButton,onClick:u,disabled:c,children:`Retry`})]}),c&&(0,z.jsxs)(`p`,{className:J.loading,children:[(0,z.jsx)(`span`,{className:`${J.spinner} ${J.spinnerDark}`,"aria-hidden":`true`}),` Reading the contract…`]}),!D&&!c&&(0,z.jsxs)(`div`,{className:J.warningBanner,role:`status`,children:[`This graph has no `,(0,z.jsx)(`code`,{children:O}),` node; the `,el[r],` contract is declared on it. Create the node, then Save.`]}),k.length>0&&(0,z.jsx)(`ul`,{className:J.issueList,"aria-label":`Declaration versus model`,children:k.map(e=>(0,z.jsxs)(`li`,{children:[`⚠️ `,e]},e))}),te(`body`),te(`header`),(0,z.jsxs)(`p`,{className:J.hint,children:[`Types are strict JSON types: a numeric string is not a number; money is a `,(0,z.jsx)(`code`,{children:`string`}),` with a `,(0,z.jsx)(`code`,{children:`pattern`}),`. A row's other keywords (`,(0,z.jsx)(`code`,{children:`enum`}),`, `,(0,z.jsx)(`code`,{children:`minimum`}),`, `,(0,z.jsx)(`code`,{children:`pattern`}),`, …) are kept as declared and edited in the node editor.`]}),w&&(0,z.jsx)(`div`,{className:J.errorBanner,role:`alert`,children:(0,z.jsxs)(`span`,{children:[`❌ `,w]})})]}),(0,z.jsxs)(`footer`,{className:J.footer,children:[(0,z.jsxs)(`div`,{className:J.footerTools,children:[(0,z.jsx)(`button`,{type:`button`,className:J.secondaryButton,onClick:()=>{y()},disabled:a||c,"aria-busy":b,title:`Types and examples from the instance's actual input and output, after a dry run`,children:b?(0,z.jsxs)(z.Fragment,{children:[(0,z.jsx)(`span`,{className:`${J.spinner} ${J.spinnerDark}`,"aria-hidden":`true`}),` Reading…`]}):`Fill from last run`}),(0,z.jsx)(`button`,{type:`button`,className:J.secondaryButton,onClick:u,disabled:a||c,title:`Read the graph again, discarding unsaved rows`,children:`Reload`}),(0,z.jsx)(`button`,{type:`button`,className:J.secondaryButton,onClick:()=>{ee()},disabled:a||c,"aria-busy":T,title:`The draft's OpenAPI 3.0 document as YAML - ${n?`the next dialog picks the folder`:`saved to the download folder`}`,children:T?(0,z.jsxs)(z.Fragment,{children:[(0,z.jsx)(`span`,{className:`${J.spinner} ${J.spinnerDark}`,"aria-hidden":`true`}),` Fetching…`]}):`Download YAML`})]}),(0,z.jsxs)(`div`,{className:J.footerActions,children:[(0,z.jsx)(`button`,{type:`button`,className:J.secondaryButton,onClick:o,disabled:a,children:`Cancel`}),(0,z.jsx)(`button`,{type:`button`,className:J.primaryButton,onClick:x,disabled:!S,"aria-busy":C,title:S?`Writes the declaration to node ${O} through update node - every member sees it`:p[r]?`Fix the flagged rows first`:`Nothing changed`,children:C?(0,z.jsxs)(z.Fragment,{children:[(0,z.jsx)(`span`,{className:J.spinner,"aria-hidden":`true`}),` Saving…`]}):`Save ${O} schema`})]})]})]})})}var ol={popover:`_popover_qw337_7`,header:`_header_qw337_19`,endpoint:`_endpoint_qw337_30`,arrow:`_arrow_qw337_41`,chips:`_chips_qw337_46`,chip:`_chip_qw337_46`,customRow:`_customRow_qw337_73`,input:`_input_qw337_78`,submitButton:`_submitButton_qw337_100`,errorText:`_errorText_qw337_122`,statusText:`_statusText_qw337_128`,warningText:`_warningText_qw337_129`},sl=12;function cl({formState:e,phase:t,lockReason:n,serverMessage:r,validationErrors:i,anchor:a,onFormStateChange:o,onSubmit:s,onClose:c}){let l=(0,P.useRef)(null),u=(0,P.useRef)(null),[d,f]=(0,P.useState)(null),p=t===`sending`,m=n===`disconnected`,h=p||m,g=(0,P.useRef)(null);(0,P.useEffect)(()=>{g.current!==null&&e.relation===g.current&&(g.current=null,h||s())},[h,e.relation,s]),(0,P.useEffect)(()=>{u.current?.focus()},[]),(0,P.useLayoutEffect)(()=>{let e={x:window.innerWidth/2,y:96},t=a??e,n=l.current?.getBoundingClientRect(),r=n?.width??280,i=n?.height??180;f({left:Math.min(Math.max(t.x-r/2,sl),Math.max(sl,window.innerWidth-r-sl)),top:Math.min(Math.max(t.y+14,sl),Math.max(sl,window.innerHeight-i-sl))})},[a]),(0,P.useEffect)(()=>{let e=e=>{e.key===`Escape`&&(e.preventDefault(),p||c())},t=e=>{p||l.current&&!l.current.contains(e.target)&&c()};return document.addEventListener(`keydown`,e),document.addEventListener(`pointerdown`,t),()=>{document.removeEventListener(`keydown`,e),document.removeEventListener(`pointerdown`,t)}},[c,p]);let _=(0,P.useCallback)(t=>{o({...e,relation:t})},[e,o]),v=(0,P.useCallback)(t=>{h||(g.current=t,o({...e,relation:t}))},[h,e,o]),y=(0,P.useCallback)(e=>{e.preventDefault(),!h&&s()},[h,s]),b=i.relation??i.command??i.sourceAlias??i.targetAlias;return(0,z.jsxs)(`div`,{ref:l,className:ol.popover,style:d?{left:d.left,top:d.top}:{visibility:`hidden`},role:`dialog`,"aria-label":`Create connection from ${e.sourceAlias} to ${e.targetAlias}`,children:[(0,z.jsxs)(`div`,{className:ol.header,children:[(0,z.jsx)(`span`,{className:ol.endpoint,children:e.sourceAlias}),(0,z.jsx)(`span`,{className:ol.arrow,"aria-hidden":`true`,children:`→`}),(0,z.jsx)(`span`,{className:ol.endpoint,children:e.targetAlias})]}),(0,z.jsx)(`div`,{className:ol.chips,children:bi.map(e=>(0,z.jsx)(`button`,{type:`button`,className:ol.chip,style:{"--chip-color":xi[e]},disabled:h,onClick:()=>v(e),children:e},e))}),(0,z.jsxs)(`form`,{className:ol.customRow,onSubmit:y,children:[(0,z.jsx)(`input`,{ref:u,className:ol.input,value:e.relation,placeholder:`custom relation…`,autoComplete:`off`,autoCorrect:`off`,spellCheck:!1,disabled:h,"aria-label":`Relation name`,"aria-invalid":!!i.relation,onChange:e=>_(e.target.value)}),(0,z.jsx)(`button`,{type:`submit`,className:ol.submitButton,disabled:h||!e.relation.trim(),children:p?`Creating…`:`Connect`})]}),b&&!p&&(0,z.jsx)(`div`,{className:ol.errorText,role:`alert`,children:b}),r&&(0,z.jsx)(`div`,{className:m?ol.warningText:ol.statusText,role:`status`,children:r})]})}var ll=1e4,ul=`A graph authoring action is already pending. Wait for it to finish before starting another.`,dl=`Could not send the create-node command because the WebSocket is not open. The form values remain in this dialog.`,fl=`Could not send the edit-node command because the WebSocket is not open. Your changes remain in this dialog.`,pl=`Could not send the delete-node command because the WebSocket is not open.`,ml=`Could not send the create-connection command because the WebSocket is not open. The form values remain in this dialog.`,hl=`Could not send delete-node commands because the WebSocket is not open.`,gl=`No selected nodes are available to delete.`,_l=`Select 100 or fewer nodes to delete at once.`,vl=`Some delete-node commands were sent, but not all backend results were observed yet. Refresh the graph before trying again.`,yl=`This node is no longer available in the current graph.`,bl=`Connection disconnected. Refresh the page and create the node again after the app reconnects.`,xl=`Connection disconnected. Refresh the page and edit the node again after the app reconnects.`,Sl=`Connection disconnected. Refresh the page and create the connection again after the app reconnects.`,Cl=`Connection disconnected while the graph authoring action was pending. The outcome is unknown. Refresh the page and check the graph before trying again.`,wl={status:`closed`,pendingSubmit:null,serverMessage:null};function Tl(e){return e.pendingSubmit}function El(e){return e.action===`delete-nodes`}function Dl(e){return e===`create-connection`?ml:e===`edit-node`?fl:e===`delete-node`?pl:dl}function Ol(e){return El(e)?vl:`The ${e.action} command was sent, but no backend result was observed yet. The outcome is unknown.`}function kl(e){return e===`create-connection`?Sl:e===`edit-node`?xl:bl}function Al(e,t){return!e||!t?!1:e.trim().toLowerCase()===t.trim().toLowerCase()}function jl(e,t){return e?.nodes.find(e=>e.alias.toLowerCase()===t.toLowerCase())??null}function Ml(e,t){return e.status===`error`?!0:t.action===`create-connection`?e.action===`create-connection`?e.alias===null?!0:Al(e.alias,t.alias)?e.status===`accepted`?Al(e.targetAlias,t.targetAlias):e.targetAlias===null||Al(e.targetAlias,t.targetAlias):!1:e.action===null?Al(e.alias,t.alias)||Al(e.alias,t.targetAlias):!1:Al(e.alias,t.alias)?e.action===null||e.action===t.action:!1}function Nl(e,t){let n=Tl(e);return!n||El(n)||!Ml(t,n)?null:t.status===`accepted`?{state:wl,acceptedResult:{status:t.status,action:t.action,alias:t.alias,targetAlias:t.targetAlias,message:t.message}}:e.status===`open`?{state:{...e,phase:`editing`,pendingSubmit:null,serverMessage:t.status===`error`?`Backend returned an error while this submit was pending: ${t.message}`:t.message}}:{state:wl,notification:{message:t.message,type:`error`}}}function Pl(e){return{...e,phase:`editing`,pendingSubmit:null,serverMessage:Dl(e.action)}}function Fl(e){let t=Tl(e);if(!t)return null;let n=Ol(t);return e.status===`open`?{state:{...e,phase:`editing`,pendingSubmit:null,serverMessage:n}}:{state:wl,notification:{message:n,type:`error`}}}function Il(e){let t=Tl(e);if(e.status===`open`){let n=t?Cl:kl(e.action);return{state:{...e,phase:`editing`,pendingSubmit:null,serverMessage:n,connectionLost:!0}}}return t?{state:wl,notification:{message:Cl,type:`error`}}:null}function Ll(e){return`sourceAlias`in e}function Y({bus:e,connected:t,graphData:n,executor:r,timeoutMs:i=ll,onAccepted:a,onUserMessage:o}){let[s,c]=(0,P.useState)(wl),[l,u]=(0,P.useState)({}),d=(0,P.useRef)(s),f=(0,P.useRef)(null),p=(0,P.useRef)(t),m=(0,P.useRef)(n),h=(0,P.useRef)(a),g=(0,P.useRef)(o);(0,P.useEffect)(()=>{d.current=s},[s]),(0,P.useEffect)(()=>{m.current=n},[n]),(0,P.useEffect)(()=>{h.current=a},[a]),(0,P.useEffect)(()=>{g.current=o},[o]);let _=(0,P.useCallback)((e,t=`error`)=>{g.current?.(e,t)},[]),v=(0,P.useCallback)(e=>{d.current=e,c(e)},[]),y=(0,P.useCallback)(()=>{f.current!==null&&(clearTimeout(f.current),f.current=null)},[]),b=(0,P.useCallback)(()=>{y(),f.current=setTimeout(()=>{let e=Fl(d.current);e&&(v(e.state),e.notification&&_(e.notification.message,e.notification.type)),f.current=null},i)},[y,_,v,i]),x=(0,P.useCallback)(e=>{if(!t)return;if(Tl(d.current)){_(ul,`error`);return}let n=tr(e);u({}),v({status:`open`,action:`create-node`,phase:`editing`,formState:n,originalAlias:null,pendingSubmit:null,serverMessage:null,connectionLost:!1})},[t,_,v]),S=(0,P.useCallback)((e,n)=>{if(!t)return;if(Tl(d.current)){_(ul,`error`);return}let r={sourceAlias:e,targetAlias:n,relation:``},{relation:i,...a}=Zn(r,{graphData:m.current,connected:t}).errors;Object.keys(a).length>0||(u({}),v({status:`open`,action:`create-connection`,phase:`editing`,formState:r,pendingSubmit:null,serverMessage:null,connectionLost:!1}))},[t,_,v]),C=(0,P.useCallback)(e=>{if(!t){_(xl,`error`);return}if(Tl(d.current)){_(ul,`error`);return}let n=jl(m.current,e.alias);if(!n){_(yl,`error`);return}let r=ur(n);if(!r.valid||!r.formState){_(r.message??`This node cannot be edited in the UI.`,`error`);return}u({}),v({status:`open`,action:`edit-node`,phase:`editing`,formState:r.formState,originalAlias:n.alias,pendingSubmit:null,serverMessage:null,connectionLost:!1})},[t,_,v]),w=(0,P.useCallback)(e=>{if(!t){_(pl,`error`);return}if(Tl(d.current)){_(ul,`error`);return}let n=Jn(e.alias,{graphData:m.current});if(!n.valid){_(Object.values(n.errors)[0]??`Invalid node alias.`,`error`);return}let i;try{i=gr(e.alias,{graphData:m.current})}catch(e){_(e instanceof Error?e.message:String(e),`error`);return}if(!r.execute(i)){_(pl,`error`);return}let a={action:`delete-node`,alias:e.alias.trim(),command:i,sentAt:new Date().toISOString()};u({}),v({status:`closed`,pendingSubmit:a,serverMessage:null}),b()},[t,r,_,v,b]),ee=(0,P.useCallback)(e=>{if(!t){_(hl,`error`);return}if(Tl(d.current)){_(ul,`error`);return}if(e.length===0){_(gl,`info`);return}if(e.length>100){_(_l,`error`);return}let n=new Set,i=e.filter(e=>{let t=e.alias.trim().toLowerCase();return n.has(t)?!1:(n.add(t),!0)}),a=[],o=[];for(let e of i){let t=Jn(e.alias,{graphData:m.current});if(!t.valid){_(Object.values(t.errors)[0]??gl,`error`);return}try{o.push(e.alias.trim()),a.push(gr(e.alias,{graphData:m.current}))}catch(e){_(e instanceof Error?e.message:String(e),`error`);return}}for(let[e,t]of a.entries())if(!r.execute(t)){_(e===0?hl:Cl,`error`);return}let s={action:`delete-nodes`,aliases:o,commands:a,sentAt:new Date().toISOString(),results:{}};u({}),v({status:`closed`,pendingSubmit:s,serverMessage:null}),_(`${o.length} delete-node commands sent. Waiting for backend response.`,`info`),b()},[t,r,_,v,b]),T=(0,P.useCallback)(e=>{let t=d.current;if(t.status===`open`&&!(t.phase===`sending`||t.connectionLost)){if(u({}),t.action===`create-connection`){if(!Ll(e))return;v({...t,formState:e,pendingSubmit:null,serverMessage:null,connectionLost:!1});return}Ll(e)||v({...t,formState:e,pendingSubmit:null,serverMessage:null,connectionLost:!1})}},[v]),E=(0,P.useCallback)(()=>{let e=d.current;if(e.status!==`open`||e.phase===`sending`||e.connectionLost)return;let n=e.action;if(!t){v({...e,serverMessage:Dl(n)});return}let i=e.action===`create-connection`?Zn(e.formState,{graphData:m.current,connected:t}):qn(e.formState,e.action===`edit-node`?{mode:`edit`,originalAlias:e.originalAlias}:{graphData:m.current});if(!i.valid){u(i.errors);return}let a,o,s=null;try{if(e.action===`edit-node`)o=e.originalAlias?.trim()??``,a=hr(e.formState,o);else if(e.action===`create-node`)o=e.formState.alias.trim(),a=mr(e.formState);else if(Ll(e.formState))o=e.formState.sourceAlias.trim(),s=e.formState.targetAlias.trim(),a=vr(e.formState);else{u({command:`Invalid connection form state.`});return}}catch(e){u({command:e instanceof Error?e.message:String(e)});return}if(!r.execute(a)){v(Pl(e));return}let c={action:n,alias:o,targetAlias:s,command:a,sentAt:new Date().toISOString()};u({}),v({...e,phase:`sending`,pendingSubmit:c,serverMessage:null,connectionLost:!1}),b()},[t,r,v,b]),D=(0,P.useCallback)(()=>{let e=d.current;e.status===`open`&&e.phase!==`sending`&&(y(),u({}),v(wl))},[y,v]);return(0,P.useEffect)(()=>e.on(`minigraph.nodeAction.textResult`,e=>{let t=d.current,n=Tl(t);if(!n)return;if(El(n)){let r=Ir(n,e);if(!r)return;if(e.status===`accepted`&&h.current?.({status:e.status,action:e.action,alias:e.alias,targetAlias:e.targetAlias,message:e.message}),!Lr(r)){v({...t,pendingSubmit:r});return}y(),v(wl);let i=Rr(r);_(i.message,i.type);return}let r=Nl(t,e);r&&(y(),r.acceptedResult&&u({}),v(r.state),r.acceptedResult&&h.current?.(r.acceptedResult),r.notification&&_(r.notification.message,r.notification.type))}),[e,y,_,v]),(0,P.useEffect)(()=>{if(p.current&&!t){let e=Il(d.current);e&&(y(),v(e.state),e.notification&&_(e.notification.message,e.notification.type))}p.current=t},[y,t,_,v]),(0,P.useEffect)(()=>()=>{y()},[y]),{state:s,validationErrors:l,openCreateNode:x,openCreateConnection:S,openEditNode:C,deleteNode:w,deleteNodes:ee,updateFormState:T,submit:E,close:D}}var X=/^ws-\d+-\d+$/;function Rl(e){return X.test(e.trim())}var zl={sessionId:null,startedSince:null,subscribedTo:null,subscribers:[],loading:!1,pendingCommand:null,error:null,lastInfo:null};function Bl(e){return Array.from(new Set(e)).sort()}function Vl({enabled:e,connected:t,bus:n,classificationMap:r,sendRawText:i,addToast:a}){let[o,s]=(0,P.useState)(zl),c=(0,P.useRef)(new Set),l=(0,P.useRef)(0),u=(0,P.useRef)(i),d=(0,P.useRef)(a);(0,P.useEffect)(()=>{u.current=i},[i]),(0,P.useEffect)(()=>{d.current=a},[a]);let f=(0,P.useCallback)(()=>{if(!e||!t)return!1;s(e=>({...e,loading:!0,pendingCommand:`refresh`,error:null,lastInfo:null}));let n=u.current(`session`);if(!n){let e=`Could not load session details because the WebSocket is not open.`;s(t=>({...t,loading:!1,pendingCommand:null,error:e})),d.current(e,`error`)}return n},[t,e]),p=(0,P.useCallback)(e=>{let t=`${e.kind}:${e.msgId}`;if(!c.current.has(t)){if(c.current.add(t),e.kind===`minigraph.session.started`){s({...zl,sessionId:e.sessionId});return}if(e.kind===`minigraph.session.status`){s(t=>({...t,sessionId:e.sessionId,startedSince:e.startedSince,subscribedTo:e.subscribedTo,subscribers:Bl(e.subscribers),loading:!1,pendingCommand:null,error:null,lastInfo:null}));return}if(e.kind===`minigraph.session.commandResult`){if(e.status===`accepted`){s(t=>({...t,subscribedTo:e.command===`subscribe`?e.sessionId:e.command===`unsubscribe`?null:t.subscribedTo,pendingCommand:null,error:null,lastInfo:null}));return}s(t=>({...t,pendingCommand:null,error:e.message,lastInfo:null}));return}if(e.kind===`minigraph.session.notification`){e.type===`host-closed`?s(t=>({...t,subscribedTo:t.subscribedTo===e.sessionId?null:t.subscribedTo,subscribers:t.subscribers.filter(t=>t!==e.sessionId),error:null,lastInfo:null})):e.type===`subscriber-joined`?s(t=>({...t,subscribers:Bl([...t.subscribers,e.sessionId]),error:null,lastInfo:null})):s(t=>({...t,subscribers:t.subscribers.filter(t=>t!==e.sessionId),error:null,lastInfo:null}));return}e.kind===`session.reset`&&(s(e=>({...e,startedSince:null,subscribedTo:null,subscribers:[],loading:!1,pendingCommand:null,error:null,lastInfo:null})),f())}},[f]),m=(0,P.useCallback)(()=>{s(e=>({...e,error:null,lastInfo:null}))},[]),h=(0,P.useCallback)(n=>{let r=n.trim();if(!e||!t||o.pendingCommand!==null||o.subscribedTo!==null)return!1;if(!Rl(r))return s(e=>({...e,error:`Enter a valid session ID like ws-123456-1.`,lastInfo:null})),!1;s(e=>({...e,pendingCommand:`subscribe`,error:null,lastInfo:null}));let c=i(`session subscribe ${r}`);if(!c){let e=`Could not subscribe because the WebSocket is not open.`;s(t=>({...t,pendingCommand:null,error:e})),a(e,`error`)}return c},[a,t,e,i,o.pendingCommand,o.subscribedTo]),g=(0,P.useCallback)(()=>{if(!e||!t||o.pendingCommand!==null||o.subscribedTo===null)return!1;s(e=>({...e,pendingCommand:`unsubscribe`,error:null,lastInfo:null}));let n=i(`session unsubscribe`);if(!n){let e=`Could not unsubscribe because the WebSocket is not open.`;s(t=>({...t,pendingCommand:null,error:e})),a(e,`error`)}return n},[a,t,e,i,o.pendingCommand,o.subscribedTo]),_=(0,P.useCallback)(()=>{if(!e||!t||o.pendingCommand!==null||o.subscribedTo!==null||o.subscribers.length===0)return!1;s(e=>({...e,pendingCommand:`reset`,error:null,lastInfo:null}));let n=i(`session reset`);if(!n){let e=`Could not reset because the WebSocket is not open.`;s(t=>({...t,pendingCommand:null,error:e})),a(e,`error`)}return n},[a,t,e,i,o.pendingCommand,o.subscribedTo,o.subscribers.length]);(0,P.useEffect)(()=>{e&&t||(c.current.clear(),l.current=0,s(zl))},[t,e]),(0,P.useEffect)(()=>{if(!e)return;let t=n.on(`minigraph.session.started`,e=>{p(e)}),r=n.on(`minigraph.session.status`,e=>{p(e)}),i=n.on(`minigraph.session.commandResult`,e=>{p(e)}),a=n.on(`minigraph.session.notification`,e=>{p(e)}),o=n.on(`session.reset`,e=>{p(e)});return()=>{t(),r(),i(),a(),o()}},[n,e,p]),(0,P.useEffect)(()=>{!e||!t||f()},[t,e,f]),(0,P.useEffect)(()=>{if(!e||!r)return;let t=l.current;for(let[e,n]of r)if(!(e<=l.current)){for(let e of n)(e.kind===`minigraph.session.started`||e.kind===`minigraph.session.status`||e.kind===`minigraph.session.commandResult`||e.kind===`minigraph.session.notification`||e.kind===`session.reset`)&&p(e);t=Math.max(t,e)}l.current=t,c.current.clear()},[r,e,p]);let v=o.subscribedTo===null,y=o.subscribers.length>0;return(0,P.useMemo)(()=>({state:o,connected:t,isPrimary:v,hasSubscribers:y,canSubscribe:e&&t&&o.pendingCommand===null&&o.subscribedTo===null,canUnsubscribe:e&&t&&o.subscribedTo!==null&&o.pendingCommand===null,canReset:e&&t&&o.pendingCommand===null&&o.subscribedTo===null&&o.subscribers.length>0,subscribeToSession:h,unsubscribe:g,resetSession:_,clearMessage:m}),[m,t,e,y,v,_,o,h,g])}var Hl=(e,t)=>t.some(t=>e instanceof t),Ul,Wl;function Gl(){return Ul||=[IDBDatabase,IDBObjectStore,IDBIndex,IDBCursor,IDBTransaction]}function Kl(){return Wl||=[IDBCursor.prototype.advance,IDBCursor.prototype.continue,IDBCursor.prototype.continuePrimaryKey]}var ql=new WeakMap,Jl=new WeakMap,Yl=new WeakMap;function Xl(e){let t=new Promise((t,n)=>{let r=()=>{e.removeEventListener(`success`,i),e.removeEventListener(`error`,a)},i=()=>{t(nu(e.result)),r()},a=()=>{n(e.error),r()};e.addEventListener(`success`,i),e.addEventListener(`error`,a)});return Yl.set(t,e),t}function Zl(e){if(ql.has(e))return;let t=new Promise((t,n)=>{let r=()=>{e.removeEventListener(`complete`,i),e.removeEventListener(`error`,a),e.removeEventListener(`abort`,a)},i=()=>{t(),r()},a=()=>{n(e.error||new DOMException(`AbortError`,`AbortError`)),r()};e.addEventListener(`complete`,i),e.addEventListener(`error`,a),e.addEventListener(`abort`,a)});ql.set(e,t)}var Ql={get(e,t,n){if(e instanceof IDBTransaction){if(t===`done`)return ql.get(e);if(t===`store`)return n.objectStoreNames[1]?void 0:n.objectStore(n.objectStoreNames[0])}return nu(e[t])},set(e,t,n){return e[t]=n,!0},has(e,t){return e instanceof IDBTransaction&&(t===`done`||t===`store`)||t in e}};function $l(e){Ql=e(Ql)}function eu(e){return Kl().includes(e)?function(...t){return e.apply(ru(this),t),nu(this.request)}:function(...t){return nu(e.apply(ru(this),t))}}function tu(e){return typeof e==`function`?eu(e):(e instanceof IDBTransaction&&Zl(e),Hl(e,Gl())?new Proxy(e,Ql):e)}function nu(e){if(e instanceof IDBRequest)return Xl(e);if(Jl.has(e))return Jl.get(e);let t=tu(e);return t!==e&&(Jl.set(e,t),Yl.set(t,e)),t}var ru=e=>Yl.get(e);function iu(e,t,{blocked:n,upgrade:r,blocking:i,terminated:a}={}){let o=indexedDB.open(e,t),s=nu(o);return r&&o.addEventListener(`upgradeneeded`,e=>{r(nu(o.result),e.oldVersion,e.newVersion,nu(o.transaction),e)}),n&&o.addEventListener(`blocked`,e=>n(e.oldVersion,e.newVersion,e)),s.then(e=>{a&&e.addEventListener(`close`,()=>a()),i&&e.addEventListener(`versionchange`,e=>i(e.oldVersion,e.newVersion,e))}).catch(()=>{}),s}function au(e,{blocked:t}={}){let n=indexedDB.deleteDatabase(e);return t&&n.addEventListener(`blocked`,e=>t(e.oldVersion,e)),nu(n).then(()=>void 0)}var ou=[`get`,`getKey`,`getAll`,`getAllKeys`,`count`],su=[`put`,`add`,`delete`,`clear`],cu=new Map;function lu(e,t){if(!(e instanceof IDBDatabase&&!(t in e)&&typeof t==`string`))return;if(cu.get(t))return cu.get(t);let n=t.replace(/FromIndex$/,``),r=t!==n,i=su.includes(n);if(!(n in(r?IDBIndex:IDBObjectStore).prototype)||!(i||ou.includes(n)))return;let a=async function(e,...t){let a=this.transaction(e,i?`readwrite`:`readonly`),o=a.store;return r&&(o=o.index(t.shift())),(await Promise.all([o[n](...t),i&&a.done]))[0]};return cu.set(t,a),a}$l(e=>({...e,get:(t,n,r)=>lu(t,n)||e.get(t,n,r),has:(t,n)=>!!lu(t,n)||e.has(t,n)}));var uu=[`continue`,`continuePrimaryKey`,`advance`],du={},fu=new WeakMap,pu=new WeakMap,mu={get(e,t){if(!uu.includes(t))return e[t];let n=du[t];return n||=du[t]=function(...e){fu.set(this,pu.get(this)[t](...e))},n}};async function*hu(...e){let t=this;if(t instanceof IDBCursor||(t=await t.openCursor(...e)),!t)return;t=t;let n=new Proxy(t,mu);for(pu.set(n,t),Yl.set(n,ru(t));t;)yield n,t=await(fu.get(n)||t.continue()),fu.delete(n)}function gu(e,t){return t===Symbol.asyncIterator&&Hl(e,[IDBIndex,IDBObjectStore,IDBCursor])||t===`iterate`&&Hl(e,[IDBIndex,IDBObjectStore])}$l(e=>({...e,get(t,n,r){return gu(t,n)?hu:e.get(t,n,r)},has(t,n){return gu(t,n)||e.has(t,n)}}));var _u=`minigraph-clipboard`,vu=1,yu=`items`,bu=null;function xu(){return iu(_u,vu,{upgrade(e){e.objectStoreNames.contains(yu)&&e.deleteObjectStore(yu);let t=e.createObjectStore(yu,{keyPath:`id`});t.createIndex(`by-alias`,`node.alias`,{unique:!0}),t.createIndex(`by-clippedAt`,`clippedAt`)}})}function Su(){return bu||=xu().catch(async e=>(console.warn(`[clipboard/db] openDB failed, deleting and recreating:`,e),bu=null,await au(_u),xu())),bu}async function Cu(){return(await(await Su()).getAllFromIndex(yu,`by-clippedAt`)).reverse()}async function wu(e){return(await Su()).getFromIndex(yu,`by-alias`,e)}async function Tu(e){await(await Su()).add(yu,e)}async function Eu(e,t){let n=(await Su()).transaction(yu,`readwrite`);await n.store.delete(e),await n.store.add(t),await n.done}async function Du(e){await(await Su()).delete(yu,e)}async function Ou(){await(await Su()).clear(yu)}var ku=`minigraph-clipboard-sync`;function Au(){return new BroadcastChannel(ku)}function ju(e,t){switch(t.type){case`HYDRATE`:return{items:t.items,isLoading:!1};case`ITEM_ADDED`:return{...e,items:[t.item,...e.items]};case`ITEM_REPLACED`:{let n=e.items.filter(e=>e.id!==t.previousId);return{...e,items:[t.item,...n]}}case`ITEM_REMOVED`:return{...e,items:e.items.filter(e=>e.id!==t.id)};case`ITEMS_CLEARED`:return{...e,items:[]};default:return e}}var Mu=(0,P.createContext)(null);function Nu({children:e}){let[t,n]=(0,P.useReducer)(ju,{items:[],isLoading:!0}),r=(0,P.useRef)(null);(0,P.useEffect)(()=>{Cu().then(e=>n({type:`HYDRATE`,items:e}))},[]),(0,P.useEffect)(()=>{let e;try{e=Au()}catch{return}return r.current=e,e.onmessage=e=>{let t=e.data;switch(t.type){case`item-added`:n({type:`ITEM_ADDED`,item:t.item});break;case`item-replaced`:n({type:`ITEM_REPLACED`,item:t.item,previousId:t.previousId});break;case`item-removed`:n({type:`ITEM_REMOVED`,id:t.id});break;case`items-cleared`:n({type:`ITEMS_CLEARED`});break}},()=>{e.close(),r.current=null}},[]);let i=(0,P.useCallback)(e=>{r.current?.postMessage(e)},[]),a=(0,P.useCallback)(async(e,t,r)=>{try{let a={id:crypto.randomUUID(),clippedAt:new Date().toISOString(),sourceWsPath:r.sourceWsPath,sourceLabel:r.sourceLabel,node:e,connections:t},o=await wu(e.alias);if(o)return{status:`duplicate`,existingItem:o,pendingItem:a};try{await Tu(a)}catch(t){if(t instanceof DOMException&&t.name===`ConstraintError`){let t=await wu(e.alias);if(t)return{status:`duplicate`,existingItem:t,pendingItem:a}}throw t}return n({type:`ITEM_ADDED`,item:a}),i({type:`item-added`,item:a}),{status:`added`}}catch(e){return{status:`error`,message:e instanceof Error?e.message:String(e)}}},[i]),o=(0,P.useCallback)(async(e,t)=>{await Eu(t,e),n({type:`ITEM_REPLACED`,item:e,previousId:t}),i({type:`item-replaced`,item:e,previousId:t})},[i]),s=(0,P.useCallback)(async e=>{await Du(e),n({type:`ITEM_REMOVED`,id:e}),i({type:`item-removed`,id:e})},[i]),c=(0,P.useCallback)(async()=>{await Ou(),n({type:`ITEMS_CLEARED`}),i({type:`items-cleared`})},[i]);return(0,z.jsx)(Mu.Provider,{value:{items:t.items,isLoading:t.isLoading,clipNode:a,confirmReplace:o,removeItem:s,clearAll:c},children:e})}function Pu(){let e=(0,P.useContext)(Mu);if(!e)throw Error(`useClipboardContext must be used inside <ClipboardProvider>`);return e}var Fu=new Intl.Collator(void 0,{sensitivity:`base`,numeric:!0});function Iu(e){return e.node.types[0]?.trim()||`unknown`}function Lu(e,t){return Fu.compare(e,t)}function Ru(e,t){return e-t}function zu(e){return e===`recent`||e===`connections`?`descending`:`ascending`}function Bu(e,t){return t===`descending`?-e:e}function Vu(e,t){let n=t.trim();if(!n)return{missing:!0,value:``};let r=e.node.properties[n];return r==null?{missing:!0,value:``}:typeof r==`string`?{missing:!1,value:r}:typeof r==`number`||typeof r==`boolean`?{missing:!1,value:String(r)}:{missing:!1,value:JSON.stringify(r)}}function Hu(e,t,n,r){let i=Vu(e,n),a=Vu(t,n);return i.missing&&!a.missing?1:!i.missing&&a.missing?-1:Bu(Lu(i.value,a.value),r)}function Uu(e,t){let n=t.direction??zu(t.field);return e.map((e,t)=>({item:e,originalIndex:t})).sort((e,r)=>{let i=0;switch(t.field){case`type`:i=Lu(Iu(e.item),Iu(r.item));break;case`alias`:i=Lu(e.item.node.alias,r.item.node.alias);break;case`source`:i=Lu(e.item.sourceLabel,r.item.sourceLabel);break;case`connections`:i=e.item.connections.length-r.item.connections.length;break;case`property`:i=Hu(e.item,r.item,t.propertyKey??``,n);break;default:i=Date.parse(e.item.clippedAt)-Date.parse(r.item.clippedAt);break}return t.field!==`property`&&(i=Bu(i,n)),i===0?Ru(e.originalIndex,r.originalIndex):i}).map(({item:e})=>e)}function Wu(e){let t=Date.now()-new Date(e).getTime();if(t<0)return`just now`;let n=Math.floor(t/1e3);if(n<60)return`just now`;let r=Math.floor(n/60);if(r<60)return`${r} min ago`;let i=Math.floor(r/60);if(i<24)return`${i} hour${i>1?`s`:``} ago`;let a=Math.floor(i/24);return a===1?`yesterday`:a<30?`${a} days ago`:new Date(e).toLocaleDateString()}var Z={item:`_item_1ne62_1`,previewFrame:`_previewFrame_1ne62_13`,preview:`_preview_1ne62_13`,previewShell:`_previewShell_1ne62_25`,metaBlock:`_metaBlock_1ne62_29`,timestamp:`_timestamp_1ne62_35`,removeChrome:`_removeChrome_1ne62_40`,removeIcon:`_removeIcon_1ne62_71`};function Gu({item:e,onRemove:t,onOpenMenu:n,onCloseMenu:r}){let{node:i,clippedAt:a,sourceLabel:o}=e;return(0,z.jsxs)(`div`,{className:Z.item,children:[(0,z.jsxs)(`div`,{className:Z.previewFrame,children:[(0,z.jsx)(`button`,{type:`button`,className:Z.removeChrome,draggable:!1,"aria-label":`Remove node ${i.alias} from clipboard`,onClick:n=>{n.stopPropagation(),r(),t(e.id)},children:(0,z.jsx)(_s,{className:Z.removeIcon,"aria-hidden":`true`,focusable:`false`})}),(0,z.jsx)(`div`,{className:Z.preview,role:`group`,draggable:!0,onDragStart:t=>{r(),Ba(t.dataTransfer,e.id)},onContextMenu:t=>{t.preventDefault(),n(e.id,t.clientX,t.clientY)},onKeyDown:t=>{if(t.key===`ContextMenu`||t.key===`F10`&&t.shiftKey){t.preventDefault();let r=t.currentTarget.getBoundingClientRect();n(e.id,Math.round(r.left+8),Math.round(r.top+8))}},tabIndex:0,"aria-label":`Drag node ${i.alias} into the graph to paste`,children:(0,z.jsx)(`div`,{className:Z.previewShell,style:di(i.types[0]??`unknown`),children:(0,z.jsx)(hi,{alias:i.alias,nodeType:i.types[0]??`unknown`,properties:i.properties})})})]}),(0,z.jsx)(`div`,{className:Z.metaBlock,children:(0,z.jsxs)(`div`,{className:Z.timestamp,children:[`Clipped `,Wu(a),` from `,o]})})]})}var Ku={menu:`_menu_164vh_1`,menuItem:`_menuItem_164vh_12`},qu=16;function Ju(e,t,n){let r=qu,i=Math.max(qu,n-t-qu);return Math.min(Math.max(e,r),i)}function Yu({open:e,x:t,y:n,canPasteToInput:r,onPasteToInput:i,onInspect:a,onClose:o}){let s=(0,P.useRef)(null),c=(0,P.useRef)(null),l=(0,P.useRef)(null),[u,d]=(0,P.useState)({left:t,top:n});return(0,P.useLayoutEffect)(()=>{if(!e||!s.current)return;let r=s.current.getBoundingClientRect();d({left:Ju(t,r.width,window.innerWidth),top:Ju(n,r.height,window.innerHeight)})},[e,t,n]),(0,P.useEffect)(()=>{if(!e)return;r?c.current?.focus():l.current?.focus();let t=e=>{s.current&&!s.current.contains(e.target)&&o()},n=e=>{e.key===`Escape`&&(e.preventDefault(),o())},i=()=>o();return document.addEventListener(`pointerdown`,t),document.addEventListener(`keydown`,n),window.addEventListener(`scroll`,i,!0),window.addEventListener(`resize`,i),()=>{document.removeEventListener(`pointerdown`,t),document.removeEventListener(`keydown`,n),window.removeEventListener(`scroll`,i,!0),window.removeEventListener(`resize`,i)}},[e,r,o]),e?(0,z.jsxs)(`div`,{ref:s,className:Ku.menu,style:{left:u.left,top:u.top},role:`menu`,"aria-label":`Clipboard item actions`,children:[(0,z.jsx)(`button`,{ref:c,role:`menuitem`,type:`button`,className:Ku.menuItem,disabled:!r,onClick:()=>{r&&i()},children:`Paste to Input`}),(0,z.jsx)(`button`,{ref:l,role:`menuitem`,type:`button`,className:Ku.menuItem,onClick:a,children:`Inspect`})]}):null}var Q={sidebar:`_sidebar_1jo34_2`,header:`_header_1jo34_12`,headerTitle:`_headerTitle_1jo34_25`,clearBtn:`_clearBtn_1jo34_32`,sortBar:`_sortBar_1jo34_48`,sortMenuWrapper:`_sortMenuWrapper_1jo34_63`,sortMenuButton:`_sortMenuButton_1jo34_68`,sortButtonLabel:`_sortButtonLabel_1jo34_93`,sortButtonValue:`_sortButtonValue_1jo34_98`,sortButtonDirection:`_sortButtonDirection_1jo34_106`,sortButtonCaret:`_sortButtonCaret_1jo34_114`,sortButtonCaretOpen:`_sortButtonCaretOpen_1jo34_121`,sortPopover:`_sortPopover_1jo34_125`,sortGroup:`_sortGroup_1jo34_139`,propertySortRow:`_propertySortRow_1jo34_150`,sortGroupTitle:`_sortGroupTitle_1jo34_154`,sortOption:`_sortOption_1jo34_164`,propertyLabel:`_propertyLabel_1jo34_197`,propertyInput:`_propertyInput_1jo34_205`,itemList:`_itemList_1jo34_223`,loading:`_loading_1jo34_233`,emptyState:`_emptyState_1jo34_243`,emptyIcon:`_emptyIcon_1jo34_256`,emptyTitle:`_emptyTitle_1jo34_261`,emptyHint:`_emptyHint_1jo34_265`,inspectPanel:`_inspectPanel_1jo34_271`,inspectHeader:`_inspectHeader_1jo34_279`,inspectClose:`_inspectClose_1jo34_293`,inspectBody:`_inspectBody_1jo34_307`,dialog:`_dialog_1jo34_313`,dialogTitle:`_dialogTitle_1jo34_328`,dialogBody:`_dialogBody_1jo34_335`,dialogActions:`_dialogActions_1jo34_342`,cancelBtn:`_cancelBtn_1jo34_349`,replaceBtn:`_replaceBtn_1jo34_363`};function Xu(){return(0,z.jsxs)(`div`,{className:Q.emptyState,children:[(0,z.jsx)(`span`,{className:Q.emptyIcon,children:`📋`}),(0,z.jsx)(`span`,{className:Q.emptyTitle,children:`No items clipped yet.`}),(0,z.jsx)(`span`,{className:Q.emptyHint,children:`Right-click a node in the Graph view to get started.`})]})}var Zu=[{value:`recent`,label:`Recent`},{value:`type`,label:`Type`},{value:`alias`,label:`Alias`},{value:`source`,label:`Source`},{value:`connections`,label:`Connections`},{value:`property`,label:`Property`}],Qu=[{value:`ascending`,label:`Ascending`},{value:`descending`,label:`Descending`}],$u=Zu.reduce((e,t)=>({...e,[t.value]:t.label}),{});function ed({connected:e,onPasteToInput:t}){let n=(0,P.useId)(),i=(0,P.useId)(),a=(0,P.useRef)(null),s=Pu(),[c,l]=(0,P.useState)(null),[u,d]=(0,P.useState)(null),[f,p]=(0,P.useState)(`recent`),[m,h]=(0,P.useState)(zu(`recent`)),[g,_]=(0,P.useState)(``),[v,y]=(0,P.useState)(!1),b=(e,t,n)=>{d({itemId:e,x:t,y:n})},x=()=>{d(null)},S=e=>{x(),t(e)},C=e=>{x(),l(t=>t?.id===e.id?null:e)},w=e=>{x(),l(t=>t?.id===e?null:t),s.removeItem(e)},ee=()=>{x(),l(null),s.clearAll()},T=e=>{p(e),h(zu(e))};(0,P.useEffect)(()=>{let e=new Set(s.items.map(e=>e.id));u&&!e.has(u.itemId)&&d(null),c&&!e.has(c.id)&&l(null)},[s.items,u,c]),(0,P.useEffect)(()=>{if(!v)return;let e=e=>{a.current?.contains(e.target)||y(!1)},t=e=>{e.key===`Escape`&&y(!1)};return document.addEventListener(`pointerdown`,e),document.addEventListener(`keydown`,t),()=>{document.removeEventListener(`pointerdown`,e),document.removeEventListener(`keydown`,t)}},[v]);let E=(0,P.useMemo)(()=>u?s.items.find(e=>e.id===u.itemId)??null:null,[u,s.items]),D=(0,P.useMemo)(()=>Uu(s.items,{field:f,direction:m,propertyKey:g}),[s.items,m,f,g]);return(0,z.jsxs)(`div`,{className:Q.sidebar,children:[(0,z.jsxs)(`div`,{className:Q.header,children:[(0,z.jsx)(`span`,{className:Q.headerTitle,children:`Workspace`}),s.items.length>0&&(0,z.jsx)(`button`,{className:Q.clearBtn,onClick:ee,"aria-label":`Clear all workspace items`,children:`Clear`})]}),s.items.length>0&&(0,z.jsx)(`div`,{className:Q.sortBar,children:(0,z.jsxs)(`div`,{className:Q.sortMenuWrapper,ref:a,children:[(0,z.jsxs)(`button`,{type:`button`,className:Q.sortMenuButton,onClick:()=>y(e=>!e),"aria-expanded":v,"aria-controls":n,children:[(0,z.jsx)(`span`,{className:Q.sortButtonLabel,children:`Sort`}),(0,z.jsx)(`span`,{className:Q.sortButtonValue,children:$u[f]}),(0,z.jsx)(`span`,{className:Q.sortButtonDirection,children:m===`ascending`?`Asc`:`Desc`}),(0,z.jsx)(`span`,{className:`${Q.sortButtonCaret}${v?` ${Q.sortButtonCaretOpen}`:``}`,"aria-hidden":`true`,children:`▾`})]}),v&&(0,z.jsxs)(`div`,{id:n,className:Q.sortPopover,children:[(0,z.jsxs)(`div`,{className:Q.sortGroup,role:`group`,"aria-labelledby":`${n}-field-title`,children:[(0,z.jsx)(`div`,{id:`${n}-field-title`,className:Q.sortGroupTitle,children:`Sort By`}),Zu.map(e=>(0,z.jsxs)(`label`,{className:Q.sortOption,children:[(0,z.jsx)(`input`,{type:`radio`,name:`${n}-field`,value:e.value,checked:f===e.value,onChange:()=>T(e.value)}),(0,z.jsx)(`span`,{children:e.label})]},e.value))]}),f===`property`&&(0,z.jsxs)(`div`,{className:Q.propertySortRow,children:[(0,z.jsx)(`label`,{className:Q.propertyLabel,htmlFor:i,children:`Property Key`}),(0,z.jsx)(`input`,{id:i,className:Q.propertyInput,value:g,onChange:e=>_(e.target.value),placeholder:`skill`,"aria-label":`Property key to sort by`})]}),(0,z.jsxs)(`div`,{className:Q.sortGroup,role:`group`,"aria-labelledby":`${n}-direction-title`,children:[(0,z.jsx)(`div`,{id:`${n}-direction-title`,className:Q.sortGroupTitle,children:`Sort Direction`}),Qu.map(e=>(0,z.jsxs)(`label`,{className:Q.sortOption,children:[(0,z.jsx)(`input`,{type:`radio`,name:`${n}-direction`,value:e.value,checked:m===e.value,onChange:()=>h(e.value)}),(0,z.jsx)(`span`,{children:e.label})]},e.value))]})]})]})}),(0,z.jsx)(`div`,{className:Q.itemList,children:s.isLoading?(0,z.jsx)(`div`,{className:Q.loading,children:`Loading…`}):s.items.length===0?(0,z.jsx)(Xu,{}):D.map(e=>(0,z.jsx)(Gu,{item:e,onRemove:w,onOpenMenu:b,onCloseMenu:x},e.id))}),c&&(0,z.jsxs)(`div`,{className:Q.inspectPanel,children:[(0,z.jsxs)(`div`,{className:Q.inspectHeader,children:[(0,z.jsxs)(`span`,{children:[`Inspect node `,c.node.alias]}),(0,z.jsx)(`button`,{className:Q.inspectClose,onClick:()=>l(null),"aria-label":`Close inspect panel`,children:`✕`})]}),(0,z.jsx)(`div`,{className:Q.inspectBody,children:(0,z.jsx)(o,{data:{node:c.node,connections:c.connections},style:r})})]}),u&&E&&(0,z.jsx)(Yu,{open:!0,x:u.x,y:u.y,canPasteToInput:e,onPasteToInput:()=>S(E),onInspect:()=>C(E),onClose:x})]})}function td(e){let{wheelTargetRef:t,scrollRef:n,contentWrapperRef:r,currentIndex:i,totalPages:a,onNavigatePrev:o,onNavigateNext:s}=e,c=(0,P.useRef)(0),l=(0,P.useRef)(null),u=(0,P.useRef)(!1),d=(0,P.useRef)(null),f=(0,P.useRef)(o),p=(0,P.useRef)(s),m=(0,P.useRef)(i),h=(0,P.useRef)(a);(0,P.useEffect)(()=>{f.current=o}),(0,P.useEffect)(()=>{p.current=s}),(0,P.useEffect)(()=>{m.current=i}),(0,P.useEffect)(()=>{h.current=a}),(0,P.useEffect)(()=>{d.current!==null&&(clearTimeout(d.current),d.current=null),r.current&&(r.current.style.transition=`none`,r.current.style.transform=`translateY(0)`),c.current=0,l.current=null},[i]),(0,P.useEffect)(()=>{let e=t.current;if(!e)return;function i(){c.current=0,l.current=null,r.current&&(r.current.style.transition=`transform 0.28s cubic-bezier(0.25, 0.46, 0.45, 0.94)`,r.current.style.transform=`translateY(0)`)}function a(e){if(e.deltaY===0)return;let t=n.current;if(!t)return;let a=t.scrollTop<=0,o=t.scrollTop+t.clientHeight>=t.scrollHeight-1,s=e.deltaY<0,g=e.deltaY>0,_=a&&s,v=o&&g;if(!_&&!v){i();return}if(u.current)return;let y=m.current,b=h.current;if(_&&y===0||v&&y===b-1)return;let x=_?`prev`:`next`;if(l.current!==null&&l.current!==x&&i(),l.current=x,c.current+=Math.abs(e.deltaY),r.current){let e=x===`prev`?-1:1,t=c.current*(18/120),n=Math.min(t,18)*e;r.current.style.transition=`none`,r.current.style.transform=`translateY(${n}px)`}if(d.current!==null&&clearTimeout(d.current),d.current=setTimeout(i,180),c.current>=120){d.current!==null&&clearTimeout(d.current);let e=l.current;i(),u.current=!0,e===`prev`?f.current():p.current(),setTimeout(()=>{u.current=!1},650)}}return e.addEventListener(`wheel`,a,{passive:!0}),()=>{d.current!==null&&clearTimeout(d.current),e.removeEventListener(`wheel`,a)}},[])}var nd={helpRoot:`_helpRoot_18tja_2`,categoryNav:`_categoryNav_18tja_11`,categoryTabScroller:`_categoryTabScroller_18tja_21`,categoryTab:`_categoryTab_18tja_21`,categoryTabActive:`_categoryTabActive_18tja_71`,maximizeButton:`_maximizeButton_18tja_78`,closeButton:`_closeButton_18tja_100`,helpBody:`_helpBody_18tja_122`,emptyFallback:`_emptyFallback_18tja_130`,helpContent:`_helpContent_18tja_147`,topicLink:`_topicLink_18tja_226`,helpBodyContent:`_helpBodyContent_18tja_271`,chipStrip:`_chipStrip_18tja_276`,chipStripLabel:`_chipStripLabel_18tja_294`,topicChip:`_topicChip_18tja_310`,topicChipActive:`_topicChipActive_18tja_338`};function rd(e){return typeof e==`string`?e:typeof e==`number`?String(e):Array.isArray(e)?e.map(rd).join(``):P.isValidElement(e)?rd(e.props.children):``}function id(e){if(!e.trim().toLowerCase().startsWith(`help `))return null;let t=e.trim().slice(5).replace(/\s*\(.*\)\s*$/,``).trim().toLowerCase();return t.length>0?t:null}function ad({activeTopic:e,onNavigate:t,onClose:n,onToggleMaximize:r,isMaximized:i,contentProfile:a=`minigraph`}){let o=(0,P.useRef)(null),s=(0,P.useRef)(null),c=(0,P.useRef)(null),l=(0,P.useRef)(null);(0,P.useEffect)(()=>{o.current&&(o.current.scrollTop=0)},[e]),(0,P.useEffect)(()=>{let e=l.current;if(!e)return;let t=e.querySelector(`[aria-current="step"]`);t&&t.scrollIntoView({block:`nearest`,inline:`nearest`,behavior:`smooth`})},[e]);let u=(0,P.useMemo)(()=>Zt(a),[a]),d=(0,P.useMemo)(()=>nn(a),[a]),f=$t(e,a),p=(0,P.useMemo)(()=>en(f,a),[f,a]),m=p.length,h=(0,P.useMemo)(()=>u.find(e=>e.id===f)?.chipStripLabel??null,[f,u]),g=d.indexOf(e),_=g<0?0:g,v=d.length;td({wheelTargetRef:s,scrollRef:o,contentWrapperRef:c,currentIndex:_,totalPages:v,onNavigatePrev:()=>t(d[_-1]??``),onNavigateNext:()=>t(d[_+1]??d[d.length-1])});let y=qt(e,a);return(0,z.jsxs)(`div`,{className:nd.helpRoot,role:`region`,"aria-label":`Help browser`,ref:s,children:[(0,z.jsxs)(`nav`,{className:nd.categoryNav,"aria-label":`Help categories`,children:[(0,z.jsx)(`div`,{className:nd.categoryTabScroller,children:u.map(e=>(0,z.jsx)(`button`,{className:[nd.categoryTab,e.id===f?nd.categoryTabActive:``].join(` `).trim(),"aria-current":e.id===f?`true`:void 0,onClick:()=>{t(en(e.id,a)[0]??``)},children:e.label},e.id))}),r&&(0,z.jsx)(`button`,{className:nd.maximizeButton,onClick:r,"aria-label":i?`Restore help panel`:`Maximize help panel`,children:i?`⊞`:`⛶`}),n&&(0,z.jsx)(`button`,{className:nd.closeButton,onClick:n,"aria-label":`Close help panel`,children:`×`})]}),m>1&&(0,z.jsxs)(`div`,{className:nd.chipStrip,ref:l,children:[h!==null&&(0,z.jsx)(`span`,{className:nd.chipStripLabel,children:h}),p.map(n=>{let r=n===e,i=tn(n,f);return(0,z.jsx)(`button`,{className:[nd.topicChip,r?nd.topicChipActive:``].join(` `).trim(),"aria-current":r?`step`:void 0,onClick:()=>t(n),children:i},n)})]}),(0,z.jsx)(`div`,{className:nd.helpBody,ref:o,children:(0,z.jsx)(`div`,{className:nd.helpBodyContent,ref:c,children:y===null?(0,z.jsxs)(`div`,{className:nd.emptyFallback,children:[(0,z.jsxs)(`code`,{children:[`help `,e||``]}),`\xA0 not found in the local bundle.`]}):(0,z.jsx)(`div`,{className:nd.helpContent,children:(0,z.jsx)(D,{remarkPlugins:[k],components:e===``?{li:({children:e,...n})=>{let r=id(rd(e).trim());return r!==null&&qt(r,a)!==null?(0,z.jsx)(`li`,{...n,children:(0,z.jsx)(`button`,{className:nd.topicLink,"aria-label":`Open help topic: ${r}`,onClick:()=>t(r),children:e})}):(0,z.jsx)(`li`,{...n,children:e})}}:void 0,children:y})})})})]})}function od({existingItem:e,pendingItem:t,onReplace:n,onCancel:r}){let i=(0,P.useRef)(null);return(0,P.useEffect)(()=>{let e=i.current;e&&!e.open&&e.showModal()},[]),(0,z.jsxs)(`dialog`,{ref:i,className:Q.dialog,onClose:r,"aria-labelledby":`duplicate-dialog-title`,children:[(0,z.jsx)(`h2`,{id:`duplicate-dialog-title`,className:Q.dialogTitle,children:`Duplicate Node`}),(0,z.jsxs)(`p`,{className:Q.dialogBody,children:[`A clipboard item with alias `,(0,z.jsxs)(`strong`,{children:[`"`,t.node.alias,`"`]}),` already exists (clipped `,Wu(e.clippedAt),`).`]}),(0,z.jsx)(`p`,{className:Q.dialogBody,children:`Replace it with the new snapshot?`}),(0,z.jsxs)(`div`,{className:Q.dialogActions,children:[(0,z.jsx)(`button`,{className:Q.cancelBtn,onClick:r,children:`Cancel`}),(0,z.jsx)(`button`,{className:Q.replaceBtn,onClick:n,children:`Replace`})]})]})}var sd={dialog:`_dialog_18jha_2`,title:`_title_18jha_20`,body:`_body_18jha_27`,label:`_label_18jha_34`,inputRow:`_inputRow_18jha_42`,input:`_input_18jha_42`,inputInvalid:`_inputInvalid_18jha_65`,extension:`_extension_18jha_69`,hint:`_hint_18jha_75`,hintError:`_hintError_18jha_82`,actions:`_actions_18jha_86`,cancelBtn:`_cancelBtn_18jha_93`,primaryBtn:`_primaryBtn_18jha_103`};function cd({defaultGraphId:e,supportsFolderPicker:t,onConfirm:n,onCancel:r}){let i=(0,P.useRef)(null),a=(0,P.useRef)(null),[o,s]=(0,P.useState)(e),[c,l]=(0,P.useState)(!1),u=(0,P.useId)(),d=(0,P.useId)(),f=(0,P.useId)(),p=o.trim(),m=$a(p),h=c&&!m;return(0,P.useEffect)(()=>{let e=i.current;e&&!e.open&&e.showModal(),a.current?.select()},[]),(0,z.jsx)(`dialog`,{ref:i,className:sd.dialog,onClose:r,"aria-labelledby":u,children:(0,z.jsxs)(`form`,{onSubmit:e=>{e.preventDefault(),l(!0),m&&n(p)},noValidate:!0,children:[(0,z.jsx)(`h2`,{id:u,className:sd.title,children:`Download graph`}),(0,z.jsx)(`label`,{className:sd.label,htmlFor:d,children:`Graph ID`}),(0,z.jsxs)(`div`,{className:sd.inputRow,children:[(0,z.jsx)(`input`,{id:d,ref:a,className:`${sd.input}${h?` ${sd.inputInvalid}`:``}`,type:`text`,value:o,onChange:e=>{s(e.target.value),l(!0)},"aria-describedby":f,"aria-invalid":h,autoComplete:`off`,spellCheck:!1,maxLength:80}),(0,z.jsx)(`span`,{className:sd.extension,"aria-hidden":`true`,children:`.json`})]}),(0,z.jsx)(`p`,{id:f,className:`${sd.hint}${h?` ${sd.hintError}`:``}`,role:h?`alert`:void 0,children:h?`A graph id uses ${Qa}.`:`Saved as ${to(p||`graph-id`)}. The root node's name in the file becomes the graph id, as "export graph as" does. `+(t?`The next dialog picks the folder.`:`The browser saves it to its download folder.`)}),(0,z.jsxs)(`div`,{className:sd.actions,children:[(0,z.jsx)(`button`,{type:`button`,className:sd.cancelBtn,onClick:r,children:`Cancel`}),(0,z.jsx)(`button`,{type:`submit`,className:sd.primaryBtn,disabled:!m,children:`Download`})]})]})})}function ld({pending:e,onReplace:t,onCancel:n}){let r=(0,P.useRef)(null),i=(0,P.useId)();(0,P.useEffect)(()=>{let e=r.current;e&&!e.open&&e.showModal()},[]);let a=(e,t)=>`${e} ${t}${e===1?``:`s`}`,o=`${a(e.nodeCount,`node`)}, ${a(e.connectionCount,`connection`)}`+(e.name?`, named "${e.name}"`:``);return(0,z.jsxs)(`dialog`,{ref:r,className:sd.dialog,onClose:n,"aria-labelledby":i,children:[(0,z.jsx)(`h2`,{id:i,className:sd.title,children:`Replace the current graph?`}),(0,z.jsxs)(`p`,{className:sd.body,children:[`Importing `,(0,z.jsxs)(`strong`,{children:[`"`,e.fileName,`"`]}),` (`,o,`) replaces the draft in this session and in every member of a shared session; a graph instance is cleared.`]}),(0,z.jsx)(`p`,{className:sd.body,children:`This cannot be undone. Download or save the current graph first if you want to keep it.`}),(0,z.jsxs)(`div`,{className:sd.actions,children:[(0,z.jsx)(`button`,{type:`button`,className:sd.cancelBtn,onClick:n,children:`Cancel`}),(0,z.jsx)(`button`,{type:`button`,className:sd.primaryBtn,onClick:t,children:`Replace`})]})]})}function ud(e){return`/api/graph/import/${e}`}function dd({sessionId:e,connected:t,hasGraph:n,addToast:r}){let[i,a]=(0,P.useState)(null),[o,s]=(0,P.useState)(!1),c=t&&e!==null,l=(0,P.useCallback)(async t=>{if(e===null){r(`Could not import because the session id is not known yet.`,`error`);return}s(!0);try{let n=await fetch(ud(e),{method:`POST`,headers:{"Content-Type":`application/json`},body:JSON.stringify(t.model)});n.ok||r(`Import of "${t.fileName}" refused - ${await Zs(n)}`,`error`)}catch(e){r(`Import of "${t.fileName}" failed: ${e.message}`,`error`)}finally{s(!1)}},[e,r]),u=(0,P.useCallback)(async i=>{let o=Array.from(i);if(o.length===0)return;if(!t||e===null){r(`Connect first to import a graph.`,`error`);return}if(o.length>1){r(`Drop one graph file at a time.`,`error`);return}let s=o[0],c=Ts(s);if(c){r(c,`error`);return}let u;try{u=JSON.parse(await ws(s))}catch(e){r(`"${s.name}" is not valid JSON: ${e.message}`,`error`);return}let d=io(u);if(!d.ok){r(`"${s.name}" is not a graph model: ${d.error}`,`error`);return}let f={...d,fileName:s.name};if(n){a(f);return}await l(f)},[t,e,n,r,l]),d=(0,P.useCallback)(()=>{if(!i)return;let e=i;a(null),l(e)},[i,l]),f=(0,P.useCallback)(()=>a(null),[]);return{canImport:c,importFiles:u,openFilePicker:(0,P.useCallback)(()=>{if(!t||e===null){r(`Connect first to import a graph.`,`error`);return}let n=document.createElement(`input`);n.type=`file`,n.accept=`.json,application/json`,n.style.display=`none`;let i=()=>n.remove();n.addEventListener(`change`,()=>{let e=n.files;i(),e&&u(e)},{once:!0}),n.addEventListener(`cancel`,i,{once:!0}),document.body.appendChild(n),n.click()},[t,e,r,u]),pending:i,confirmPending:d,cancelPending:f,isImporting:o}}function fd(e){return Ls.map(t=>({key:e(),name:t,value:``}))}function pd({addToast:e,importFiles:t,graphData:n,graphName:r}){let i=(0,P.useRef)(1),a=(0,P.useCallback)(()=>i.current++,[]),[o,s]=(0,P.useState)(!1),[c,l]=(0,P.useState)(`assemble`),[u,d]=(0,P.useState)([]),[f,p]=(0,P.useState)([]),[m,h]=(0,P.useState)(``),[g,_]=(0,P.useState)(()=>fd(a)),[v,y]=(0,P.useState)(!1),[b,x]=(0,P.useState)(null),[S,C]=(0,P.useState)(null),[w,ee]=(0,P.useState)(!1),T=(0,P.useRef)(null),E=(0,P.useCallback)(()=>{T.current?.abort(),T.current=null},[]);(0,P.useEffect)(()=>E,[E]);let D=(0,P.useMemo)(()=>Hs(u),[u]),O=(0,P.useMemo)(()=>new Set(u.map(e=>e.id)),[u]),k=(0,P.useMemo)(()=>Us(g,O),[g,O]),A=(0,P.useMemo)(()=>Vs(m),[m]),te=u.length>0&&D.size===0&&k.size===0&&A===null&&!v&&!w,ne=(0,P.useCallback)(()=>s(!0),[]),re=(0,P.useCallback)(()=>{E(),y(!1),ee(!1),l(`assemble`),C(null),s(!1)},[E]),j=(0,P.useCallback)(async e=>{E();let t=new AbortController;T.current=t,ee(!0);try{let n=await fetch(Ms,{method:`POST`,headers:{"Content-Type":`application/octet-stream`,Accept:`application/json`},body:e,signal:t.signal});if(!n.ok){p([`"${e.name}" was not read - ${await Zs(n)}`]);return}let r=Ks(await n.json());C({fileName:e.name,set:r}),l(`inspect`)}catch(t){if(t.name===`AbortError`)return;p([`"${e.name}" was not read: ${t.message}`])}finally{T.current===t&&(T.current=null),ee(!1)}},[E]),M=(0,P.useCallback)(async e=>{let t=Array.from(e);if(t.length===0)return;x(null);let n=t.filter(zs);if(n.length>0){if(t.length>1){p([`Drop one .pack file on its own to inspect it, or .json graph files to pack.`]);return}p([]),await j(n[0]);return}let r=[],i=[];for(let e of t){let t=Ts(e);if(t){r.push(t);continue}let n;try{n=JSON.parse(await ws(e))}catch(t){r.push(`"${e.name}" is not valid JSON: ${t.message}`);continue}let o=io(n);if(!o.ok){r.push(`"${e.name}" is not a graph model: ${o.error}`);continue}i.push({key:a(),id:Rs(e.name),label:e.name,model:o.model,name:o.name,nodeCount:o.nodeCount,connectionCount:o.connectionCount,source:`file`})}p(r),i.length>0&&d(e=>[...e,...i])},[j,a]),N=n!==null,ie=(0,P.useCallback)(()=>{if(!n)return;let t=eo(r),i=io(JSON.parse(ao(n,t)));if(!i.ok){e(`The current graph cannot be packed: ${i.error}`,`error`);return}let o={key:a(),id:t,label:`current graph`,model:i.model,name:i.name,nodeCount:i.nodeCount,connectionCount:i.connectionCount,source:`draft`};x(null),d(e=>{let n=e.findIndex(e=>e.source===`draft`&&e.id===t);if(n<0)return[...e,o];let r=[...e];return r[n]=o,r})},[n,r,a,e]),F=(0,P.useCallback)(e=>{x(null),d(t=>t.filter(t=>t.key!==e))},[]),ae=(0,P.useCallback)(()=>{x(null),p([]),d([])},[]),I=(0,P.useCallback)((e,t)=>{x(null),_(n=>n.map(n=>n.key===e?{...n,...t}:n))},[]),L=(0,P.useCallback)(()=>{_(e=>[...e,{key:a(),name:``,value:``}])},[a]),oe=(0,P.useCallback)(e=>{x(null),_(t=>t.filter(t=>t.key!==e))},[]),se=(0,P.useCallback)(async()=>{if(!te)return;E();let t=new AbortController;T.current=t,x(null),y(!0);let n=m.trim();try{let r=await fetch(js,{method:`POST`,headers:{"Content-Type":`application/json`,Accept:`application/octet-stream`},body:JSON.stringify(Ws(n,g,u)),signal:t.signal});if(!r.ok){x(await Zs(r));return}let i=await fo(new Uint8Array(await r.arrayBuffer()),Bs(n));i.saved&&e(`Graph set saved as ${i.fileName}`,`success`)}catch(e){if(e.name===`AbortError`)return;x(`Packing failed: ${e.message}`)}finally{T.current===t&&(T.current=null),y(!1)}},[te,E,m,g,u,e]),ce=(0,P.useCallback)(async e=>{let n=S?.set.graphs.find(t=>t.id===e);n&&await t([new File([JSON.stringify(n.model)],`${e}.json`,{type:`application/json`})])},[S,t]),le=(0,P.useCallback)(()=>{if(!S)return;let{fileName:t,set:n}=S;d(n.graphs.map(e=>({key:a(),id:e.id,label:`${e.id}.json from ${t}`,model:e.model,name:e.name,nodeCount:e.nodeCount,connectionCount:e.connectionCount,source:`set`})));let r=qs(n.manifest,a);_(r.length>0?r:fd(a)),h(n.manifest.set??``),p([]),x(null),C(null),l(`assemble`);let i=n.graphs.length;e(`Editing ${t}: ${i} graph${i===1?``:`s`} and ${r.length} manifest field${r.length===1?``:`s`} loaded`,`info`)},[S,a,e]),ue=(0,P.useCallback)(()=>{C(null),l(`assemble`)},[]);return{isOpen:o,open:ne,close:re,mode:c,busy:v||w,entries:u,entryIssues:D,addFiles:M,fileErrors:f,canAddCurrentGraph:N,addCurrentGraph:ie,removeEntry:F,clearEntries:ae,setName:m,setSetName:h,setNameError:A,rows:g,rowIssues:k,updateRow:I,addRow:L,removeRow:oe,canPack:te,pack:se,isPacking:v,packError:b,inspected:S,isReading:w,importInspectedGraph:ce,editInspectedSet:le,leaveInspect:ue}}var md=class{constructor(){this.listeners=new Map}on(e,t){let n=e;return this.listeners.has(n)||this.listeners.set(n,new Set),this.listeners.get(n).add(t),()=>{this.listeners.get(n)?.delete(t)}}emit(e){let t=this.listeners.get(e.kind);t&&t.forEach(t=>{try{t(e)}catch(t){console.error(`[ProtocolBus] listener for '${e.kind}' threw:`,t)}})}clear(){this.listeners.clear()}},hd=`(ws-\\d+-\\d+)`,gd=RegExp(`^session ${hd} started(?:\\nCompanion endpoint: (\\/api\\/companion\\/${hd}))?$`,`i`),_d=RegExp(`^Session ${hd} started since (.+)$`),vd=RegExp(`^subscribed to ${hd}$`),$=/^subscribed by \[(.*)]$/,yd=RegExp(`^Subscribed to ${hd}$`),bd=RegExp(`^Session unsubscribed from ${hd}$`),xd=RegExp(`^Session ${hd} not found$`),Sd=RegExp(`^${hd} is not a primary session$`),Cd=RegExp(`^You have already subscribed to ${hd}(?:\\nPlease do 'session reset' before subscribing to another session)?$`),wd=RegExp(`^${hd} subscribed to your session$`),Td=RegExp(`^${hd} unsubscribed from your session$`),Ed=RegExp(`^Session ${hd} has closed$`);function Dd(e){let t=e.trim();return t.length===0||t.startsWith(`> `)?null:t}function Od(e){return e.split(`,`).map(e=>e.trim()).filter(e=>e.length>0&&Rl(e))}function kd(e){let t=Dd(e);if(!t)return null;let n=t.match(gd);return n?{sessionId:n[1],companionEndpoint:n[2]??null}:null}function Ad(e){let t=Dd(e);if(!t)return null;let n=t.split(`
`).map(e=>e.trim()).filter(Boolean),r=n[0];if(!r)return null;let i=r.match(_d);if(!i)return null;let a=null,o=[];for(let e of n.slice(1)){let t=e.match(vd);if(t){a=t[1];continue}let n=e.match($);n&&(o=Od(n[1]))}return{sessionId:i[1],startedSince:i[2],subscribedTo:a,subscribers:o}}function jd(e){let t=Dd(e);if(!t)return null;let n=t.match(yd);if(n)return{command:`subscribe`,status:`accepted`,sessionId:n[1],message:t};let r=t.match(bd);if(r)return{command:`unsubscribe`,status:`accepted`,sessionId:r[1],message:t};let i=t.match(xd);if(i)return{command:`subscribe`,status:`rejected`,sessionId:i[1],message:t};let a=t.match(Sd);if(a)return{command:`subscribe`,status:`rejected`,sessionId:a[1],message:t};let o=t.match(Cd);return o?{command:`subscribe`,status:`rejected`,sessionId:o[1],message:t}:t===`You cannot subscribe to yourself`?{command:`subscribe`,status:`rejected`,sessionId:null,message:t}:t===`Nothing to unsubscribe`?{command:`unsubscribe`,status:`rejected`,sessionId:null,message:t}:t===`Invalid session command`?{command:`unknown`,status:`rejected`,sessionId:null,message:t}:null}function Md(e){let t=Dd(e);if(!t)return null;let n=t.match(wd);if(n)return{type:`subscriber-joined`,sessionId:n[1],message:t};let r=t.match(Td);if(r)return{type:`subscriber-left`,sessionId:r[1],message:t};let i=t.match(Ed);return i?{type:`host-closed`,sessionId:i[1],message:t}:null}var Nd=new Set([`info`,`error`,`ping`,`welcome`]);function Pd(e,t){let n=[],r={msgId:e,raw:t},i=!1,a=!1,o=!1,s=!1,c=!1,l=!1,u=ye(t);if(u.isJSON){let e=u.data;if(typeof e.type==`string`){let i=e.type;return n.push({...r,kind:`lifecycle`,type:i,knownType:Nd.has(i),message:typeof e.message==`string`?e.message:t,time:e.time??null}),n.length>0?n:[{...r,kind:`unclassified`}]}return n.push({...r,kind:`json.response`,data:u.data}),n.length>0?n:[{...r,kind:`unclassified`}]}let d=Ee(t);d&&(c=!0,n.push({...r,kind:`payload.large`,apiPath:d.apiPath,byteSize:d.byteSize,filename:d.filename}));let f=De(t);f&&(o=!0,n.push({...r,kind:`upload.invitation`,uploadPath:f}));let p=Te(t);if(p&&(s=!0,n.push({...r,kind:`upload.contentPath`,uploadPath:p})),we(t)){a=!0;let e=Ce(t);e&&n.push({...r,kind:`graph.link`,apiPath:e})}if(a){let e=be(t);e&&n.push({...r,kind:`graph.exported`,graphName:e.graphName,apiPath:e.apiPath})}let m=Be(t);m&&n.push({...r,kind:`graph.mutation`,mutationType:m});let h=An(t);h&&n.push({...r,kind:`graph.instance.created`,mockEntries:h.mockEntries,ttlMs:h.ttlMs}),jn(t)&&n.push({...r,kind:`graph.instance.cleared`});let g=Mn(t);g&&n.push({...r,kind:`graph.run.terminal`,status:g.status,elapsedMs:g.elapsedMs});let _=Nn(t);_&&n.push({...r,kind:`command.error`,message:_});let v=ze(t);v&&n.push({...r,kind:`minigraph.nodeAction.textResult`,status:v.status,action:v.action,alias:v.alias,targetAlias:v.targetAlias,message:v.message}),v&&(v.action===`create-node`||v.status===`error`)&&n.push({...r,kind:`minigraph.createNode.textResult`,status:v.status,alias:v.alias,message:v.message}),t===`Session restarted`&&(l=!0,n.push({...r,kind:`session.reset`}));let y=kd(t);y&&(l=!0,n.push({...r,kind:`minigraph.session.started`,sessionId:y.sessionId,companionEndpoint:y.companionEndpoint}));let b=Ad(t);b&&(l=!0,n.push({...r,kind:`minigraph.session.status`,sessionId:b.sessionId,startedSince:b.startedSince,subscribedTo:b.subscribedTo,subscribers:b.subscribers}));let x=jd(t);x&&(l=!0,n.push({...r,kind:`minigraph.session.commandResult`,command:x.command,status:x.status,sessionId:x.sessionId,message:x.message}));let S=Md(t);S&&(l=!0,n.push({...r,kind:`minigraph.session.notification`,type:S.type,sessionId:S.sessionId,message:S.message})),t.startsWith(`> `)&&(i=!0,n.push({...r,kind:`command.echo`,commandText:t.slice(2)})),Oe(t)&&n.push({...r,kind:`command.helpOrDescribe`,commandText:t.slice(2)});let C=ke(t);C&&n.push({...r,kind:`command.importGraph`,graphName:C});let w=xe(t);return w&&n.push({...r,kind:`graph.export.failed`,reason:w.reason}),!i&&!a&&!o&&!s&&!c&&!l&&Se(t)&&n.push({...r,kind:`docs.response`,isMarkdown:!0}),n.length===0&&n.push({...r,kind:`unclassified`}),n}function Fd({messages:e,bus:t}){let n=(0,P.useRef)(-1);(0,P.useEffect)(()=>{e.length>0&&(n.current=e[e.length-1].id)},[]);let r=(0,P.useMemo)(()=>{let t=new Map;for(let n of e)t.set(n.id,Pd(n.id,n.raw));return t},[e]);return(0,P.useEffect)(()=>{if(e.length===0)return;let i=e.filter(e=>e.id>n.current);if(i.length!==0){n.current=e[e.length-1].id;for(let e of i){let n=r.get(e.id);if(n)for(let e of n)t.emit(e)}}},[e,t,r]),{classificationMap:r}}var Id={console:100/3,"node-edit":30,upload:30,package:30,schema:38},Ld=20;function Rd(e){return`${e}%`}function zd({config:e}){let{title:t,wsPath:n,storageKeyPayload:r,storageKeyHistory:i,storageKeyTab:a,storageKeySavedGraphs:o,supportsUpload:s,supportsClipboard:c,supportsHelp:l,helpContentProfile:u=`minigraph`,supportsAuthoring:d,supportsGraphRun:f,supportsSessionCollaboration:p,tabs:m}=e,h=ee(),[g,_]=ce(r,``),v=ge(),[y,b]=(0,P.useState)(()=>v.peekPendingPayload(n)),{takePendingPayload:x}=v;(0,P.useEffect)(()=>{let e=x(n);e!==null&&b(e)},[x,n]);let S=y??g,C=(0,P.useCallback)(e=>{b(null),_(e)},[_]),w=(0,P.useMemo)(()=>S?L(S):{valid:!0,error:null,type:null},[S]),{toasts:T,addToast:E,removeToast:D}=se(),O=(0,P.useRef)(new md).current,k=Ue({wsPath:n,storageKeyHistory:i,payload:S,addToast:E,bus:O,handleLocalCommand:(0,P.useCallback)(e=>rn(e,l===!0,u)!==null,[l,u])}),{classificationMap:re}=Fd({messages:k.messages,bus:O}),j=Vl({enabled:p===!0,bus:O,classificationMap:re,connected:k.connected,sendRawText:k.sendRawText,addToast:E}),M=j.state.sessionId===null?null:`/api/graph/session/${j.state.sessionId}`,{graphData:N,setGraphData:ie,rightTab:ae,setRightTab:I,isRefreshing:le,refetchGraph:ue}=Je(M,E,m[0],m,a),{uploadPanelPath:R,successfulUploadPaths:de,handleOpenUploadPanel:fe,handleCloseUploadPanel:pe,handleUploadSuccess:me,handleUploadError:he,resetSuccessfulPaths:_e}=sn({addToast:E}),ve=dd({sessionId:j.state.sessionId,connected:k.connected,hasGraph:N!==null,addToast:E}),[ye,be]=(0,P.useState)(!1),xe=(0,P.useCallback)(()=>{if(!N){E(`No graph to download yet.`,`info`);return}be(!0)},[N,E]),Se=(0,P.useCallback)(async e=>{if(be(!1),N)try{let t=await U(ao(N,e),to(e));t.saved&&E(`Graph saved as ${t.fileName}`,`success`)}catch(e){E(`Download failed: ${e instanceof Error?e.message:String(e)}`,`error`)}},[N,E]),Ce=(0,P.useCallback)(()=>ie(null),[ie]);Ye({bus:O,hasGraph:N!==null,connected:k.connected,refetchGraph:ue,clearGraph:Ce,addToast:E});let we=Pr({bus:O,connected:k.connected,sendRawText:k.sendRawText,addToast:E}),Te=(0,P.useRef)(null),Ee=(0,P.useRef)(null),De=(0,P.useRef)(new Map),Oe=(0,P.useCallback)((e,t)=>{if(t===null){E(e,`success`);return}E(e,`success`,{durationMs:6e3,action:{label:`Undo`,onClick:()=>we.undoEntry(t)}})},[E,we.undoEntry]),ke=(0,P.useCallback)(e=>{if(e.action===`edit-node`){let t=Te.current;Te.current=null,t&&t.alias===e.alias&&Oe(`Updated node ${t.alias}`,we.push(kr(t)));return}if(e.action===`create-node`&&e.alias){Oe(`Created node ${e.alias}`,we.push(Ar(e.alias)));return}if(e.action===`create-connection`){let t=Ee.current;Ee.current=null,Oe(`Connected ${e.alias??``} → ${e.targetAlias??``}`,we.push(t));return}if(e.action===`delete-node`&&e.alias){let t=De.current.get(e.alias)??null;De.current.delete(e.alias),Oe(`Deleted node ${e.alias}`,we.push(t))}},[we.push,Oe]),Ae=(0,P.useRef)(!1);(0,P.useEffect)(()=>{Ae.current&&!k.connected&&ie(null),Ae.current=k.connected},[k.connected,ie]);let[je,Me]=ce(e.storageKeyHelpTopic??`help-topic-fallback`,``),[Ne,Pe]=ce(`help-panel-open`,!1),[Fe,Ie]=(0,P.useState)(()=>!!l&&!Ne),[Le,Re]=(0,P.useState)(!1),ze=(0,P.useRef)(null),Be=(0,P.useCallback)(()=>{Fe&&(Re(!0),ze.current=setTimeout(()=>Ie(!1),400))},[Fe]);(0,P.useEffect)(()=>{if(!Fe||Le)return;let e=setTimeout(Be,3e3);return()=>clearTimeout(e)},[Fe,Le,Be]),(0,P.useEffect)(()=>{Ne&&Fe&&Be()},[Ne,Fe,Be]),(0,P.useEffect)(()=>()=>{ze.current&&clearTimeout(ze.current)},[]),(0,P.useEffect)(()=>{if(!l)return;let e=e=>{e.ctrlKey&&e.key==="`"&&(e.preventDefault(),Pe(e=>!e))};return window.addEventListener(`keydown`,e),()=>window.removeEventListener(`keydown`,e)},[l,Pe]),an({bus:O,setHelpTopic:Me,onTabSwitch:l?()=>Pe(!0):()=>{},enabled:l===!0,contentProfile:u}),cn({bus:O,connected:k.connected,appendMessage:k.appendMessage,addToast:E});let[Ve,He]=ce(`console-panel-open`,!0),Ge=Pu(),[Ke,qe]=ce(`clipboard-sidebar-open`,!1),[Xe,Ze]=(0,P.useState)(null),Qe=(0,P.useCallback)(e=>{let t;try{t=br(e,N)}catch(e){E(`Paste failed: ${e instanceof Error?e.message:String(e)}`,`error`);return}k.setCommand(t.command),E(`${t.verb===`create`?`Create`:`Update`} command for "${e.node.alias}" pasted to input`,`info`)},[N,k.setCommand,E]),$e=(0,P.useCallback)(e=>{let t=Ge.items.find(t=>t.id===e);if(!t){E(`Clipboard item is no longer available. It may have been removed in another tab.`,`error`);return}let n;try{n=br(t,N)}catch(e){E(`Paste failed: ${e instanceof Error?e.message:String(e)}`,`error`);return}if(!k.sendRawText(n.command)){E(`Could not send clipboard paste command because the WebSocket is not open.`,`error`);return}E(`Clipboard node "${t.node.alias}" sent as ${n.verb}. Waiting for backend response.`,`info`)},[Ge.items,N,k.sendRawText,E]),et=(0,P.useCallback)(async(t,r)=>{try{let i=await Ge.clipNode(t,r,{sourceWsPath:n,sourceLabel:e.label});switch(i.status){case`added`:E(`Node "${t.alias}" clipped to workspace`,`success`);break;case`duplicate`:Ze({pendingItem:i.pendingItem,existingItem:i.existingItem});break;case`error`:E(`Clip failed: ${i.message}`,`error`);break}}catch(e){E(`Clip failed: ${e instanceof Error?e.message:String(e)}`,`error`)}},[Ge,n,e.label,E]),tt=(0,P.useCallback)(async t=>{if(t.length===0){E(`No selected nodes are available to clip.`,`info`);return}if(t.length>100){E(`Select 100 or fewer nodes to clip at once.`,`error`);return}let r={added:0,duplicates:0,failed:0};for(let i of t){if(!i.node?.alias?.trim()){r.failed+=1;continue}try{let t=await Ge.clipNode(i.node,i.connections,{sourceWsPath:n,sourceLabel:e.label});t.status===`added`&&(r.added+=1),t.status===`duplicate`&&(r.duplicates+=1),t.status===`error`&&(r.failed+=1)}catch{r.failed+=1}}let i=Sr(r);E(i.message,i.type)},[E,Ge,e.label,n]),nt=ln(o??``),{defaultName:rt,savedName:it,resetName:at}=hn(o?`${o}-untitled-counter`:`untitled-counter`,O,k.connected,k.connectionEpoch),ot=(0,P.useMemo)(()=>{let e=N?.nodes.find(e=>e.types.includes(`Root`)),t=typeof e?.properties?.name==`string`?e.properties.name:void 0;return t?.trim()?t:null},[N])??rt,st=pd({addToast:E,importFiles:ve.importFiles,graphData:N,graphName:ot}),ct=Zc({addToast:E,sessionId:j.state.sessionId,connected:k.connected,graphData:N,graphName:ot,bus:O,sendRawText:k.sendRawText}),lt=(0,P.useMemo)(()=>Cr(k.sendRawText),[k.sendRawText]),ut=Y({bus:O,connected:k.connected,graphData:N,executor:lt,onAccepted:ke,onUserMessage:E}),dt=ut.state,ft=d&&dt.status===`open`&&(dt.action===`edit-node`||dt.action===`create-node`)?dt:null,pt=d&&dt.status===`open`&&dt.action===`create-connection`?dt:null,[mt,ht]=(0,P.useState)(null),gt=ft!==null||pt!==null;(0,P.useEffect)(()=>{let e=e=>{if(e.key!==`z`&&e.key!==`Z`||!(e.metaKey||e.ctrlKey)||e.shiftKey||e.altKey)return;let t=e.target;t instanceof Element&&t.closest(`input, textarea, select, [contenteditable="true"]`)||gt||we.hasEntries()&&(e.preventDefault(),we.undoLast())};return window.addEventListener(`keydown`,e),()=>window.removeEventListener(`keydown`,e)},[gt,we.hasEntries,we.undoLast]);let _t=(0,P.useCallback)((e,t,n)=>{ht(n??null),Ee.current=N?jr(N,e,t):null,ut.openCreateConnection(e,t)},[ut.openCreateConnection,N]),vt=(0,P.useCallback)(e=>{if(!N)return;let t=Tr(N,e);t&&(we.runCommands(t.commands),Oe(`Deleted ${Er(t.removed)}`,we.push(Or(t.removed))))},[N,we.push,we.runCommands,Oe]),yt=(0,P.useCallback)(e=>{Te.current=e,ut.openEditNode(e)},[ut.openEditNode]),bt=(0,P.useCallback)(e=>{De.current.set(e.alias,N?Mr(N,e):null),ut.deleteNode(e)},[ut.deleteNode,N]),xt=k.consoleRef,St=Ve&&ft===null&&R===null&&!st.isOpen&&!ct.isOpen;(0,P.useEffect)(()=>{St&&xt.current&&(xt.current.scrollTop=xt.current.scrollHeight)},[St,xt]);let Ct=Rn({enabled:f===!0,bus:O,connected:k.connected,connectionEpoch:k.connectionEpoch,graphData:N,graphIdentity:M,isPrimary:j.isPrimary,sendRawText:k.sendRawText,addToast:E}),{handleSaveGraph:wt,handleLoadGraph:Tt}=Bn({bus:O,connected:k.connected,sendRawText:k.sendRawText,saveGraph:o?nt.saveGraph:null,addToast:E}),Et=(0,P.useCallback)(e=>{re.get(e.id)?.find(e=>e.kind===`graph.link`)&&(ue(),I(`graph`))},[re,ue,I]),{handleSendToJsonPath:Dt}=on({ctx:v,navigate:h,addToast:E,wsPath:n}),Ot=We(`(max-width: 768px)`),kt=ft===null?R===null?st.isOpen?`package`:ct.isOpen?`schema`:Ve?`console`:null:`upload`:`node-edit`,At=(0,P.useRef)(null),jt=(0,P.useRef)(kt);(0,P.useEffect)(()=>{let e=jt.current;if(jt.current=kt,kt===null||kt===e)return;let t=requestAnimationFrame(()=>{At.current?.resize(Rd(Id[kt]))});return()=>cancelAnimationFrame(t)},[kt]);let Mt=!!c&&Ke,Nt=Rd(100-(kt===null?0:Id[kt])-(Mt?Ld:0)),Pt=(0,P.useCallback)(()=>C(oe(S)),[S]),Ft=(0,P.useCallback)(()=>{k.clearMessages(),ie(null),_e(),at()},[k.clearMessages,ie,_e,at]),It=(0,P.useCallback)(()=>{let e=j.state.sessionId;if(e===null){E(`Could not open the upload form because the session id is not known yet.`,`error`);return}fe(`/api/mock/${e}`)},[j.state.sessionId,fe,E]),Lt=(0,P.useCallback)(()=>{if(ft!==null){E(`Close the node editor first.`,`info`);return}R!==null&&pe(),ct.isOpen&&ct.close(),st.open()},[ft,R,pe,st.open,ct,E]),Rt=(0,P.useCallback)((e=`input`,t=!1)=>{if(ft!==null){if(!t){E(`Close the node editor first.`,`info`);return}if(ft.phase===`sending`)return;ut.close()}if(R!==null&&pe(),st.isOpen){if(st.busy){E(`Wait for the graph set to finish packing.`,`info`);return}st.close()}ct.open(e)},[ft,ut,R,pe,st,ct,E]),zt=(0,P.useCallback)(()=>Rt(`input`),[Rt]),Bt=ft!==null&&ft.action===`edit-node`?Object.keys(nc).find(e=>nc[e]===ft.formState.alias.trim())??null:null;return(0,z.jsxs)(`div`,{className:F.wrapper,children:[(0,z.jsx)(Br,{toasts:T,onRemove:D}),ye&&N&&(0,z.jsx)(cd,{defaultGraphId:eo(ot),supportsFolderPicker:oo(),onConfirm:Se,onCancel:()=>be(!1)}),ve.pending&&(0,z.jsx)(ld,{pending:ve.pending,onReplace:ve.confirmPending,onCancel:ve.cancelPending}),pt!==null&&(0,z.jsx)(cl,{formState:pt.formState,phase:pt.phase,lockReason:pt.phase===`sending`?`sending`:pt.connectionLost?`disconnected`:null,serverMessage:pt.serverMessage,validationErrors:ut.validationErrors,anchor:mt,onFormStateChange:ut.updateFormState,onSubmit:ut.submit,onClose:ut.close}),(0,z.jsxs)(`header`,{className:F.header,children:[(0,z.jsx)(`h1`,{className:F.title,children:t}),(0,z.jsxs)(`div`,{className:F.headerActions,children:[o&&(0,z.jsx)($r,{disabled:!N,defaultName:rt,savedName:it,onSave:wt,nameExists:nt.hasGraph,connected:k.connected}),o&&nt.savedGraphs.length>0&&(0,z.jsx)(ti,{savedGraphs:nt.savedGraphs,onLoad:Tt,onDelete:nt.deleteGraph,connected:k.connected}),(0,z.jsx)(`button`,{className:F.panelToggle,onClick:()=>{if(ft!==null){ft.phase!==`sending`&&(ut.close(),He(!0));return}if(R!==null){pe(),He(!0);return}if(st.isOpen){st.busy||(st.close(),He(!0));return}if(ct.isOpen){ct.busy||(ct.close(),He(!0));return}He(e=>!e)},"aria-label":ft===null?R===null?st.isOpen?`Show console panel and close the graph-set panel`:ct.isOpen?`Show console panel and close the schema panel`:Ve?`Hide console panel`:`Show console panel`:`Show console panel and close the upload form`:`Show console panel and close the node editor`,"aria-pressed":St,title:ft===null?R===null?st.isOpen?`Closes the graph-set panel`:ct.isOpen?`Closes the schema panel`:void 0:`Closes the upload form`:`Closes the node editor`,children:`Console`}),c&&(0,z.jsxs)(`button`,{className:F.panelToggle,onClick:()=>qe(e=>!e),"aria-label":Ke?`Close workspace sidebar`:`Open workspace sidebar`,"aria-pressed":Ke,children:[`Workspace`,Ge.items.length>0?` (${Ge.items.length})`:``]}),(0,z.jsx)(Xr,{addToast:E,sessionCollaboration:p?j:null,onPackageGraphs:d?Lt:void 0,onEditSchema:d?zt:void 0}),l&&(0,z.jsxs)(`div`,{className:F.helpButtonWrapper,children:[(0,z.jsx)(`button`,{className:`${F.helpToggle}${Fe&&!Le?` ${F.helpTogglePulsing}`:``}`,onClick:()=>Pe(e=>!e),"aria-label":Ne?`Close help panel`:`Open help panel`,"aria-pressed":Ne,children:`?`}),Fe&&(0,z.jsxs)(`div`,{className:`${F.helpHint}${Le?` ${F.helpHintFading}`:``}`,onClick:Be,role:`status`,children:[(0,z.jsx)(`kbd`,{className:F.helpHintKbd,children:"Ctrl + `"}),` to toggle help`]})]})]})]}),Xe&&(0,z.jsx)(od,{existingItem:Xe.existingItem,pendingItem:Xe.pendingItem,onReplace:async()=>{try{await Ge.confirmReplace(Xe.pendingItem,Xe.existingItem.id),Ze(null),E(`Clipboard item "${Xe.pendingItem.node.alias}" replaced`,`success`)}catch(e){E(`Replace failed: ${e instanceof Error?e.message:String(e)}`,`error`)}},onCancel:()=>{Ze(null),E(`Clip cancelled`,`info`)}}),(0,z.jsxs)(ne,{className:F.panelGroup,orientation:Ot?`vertical`:`horizontal`,children:[kt!==null&&(0,z.jsxs)(z.Fragment,{children:[(0,z.jsx)(A,{panelRef:At,defaultSize:Rd(Id[kt]),minSize:`25%`,children:ft===null?R===null?st.isOpen?(0,z.jsx)(Xs,{controller:st,supportsFolderPicker:oo()}):ct.isOpen?(0,z.jsx)(al,{controller:ct,graphName:ot,supportsFolderPicker:oo()}):(0,z.jsx)(gs,{messages:k.messages,classificationMap:re,onCopy:k.copyMessages,onClear:Ft,consoleRef:k.consoleRef,command:k.command,onCommandChange:k.setCommand,onCommandKeyDown:k.handleKeyDown,onSend:k.sendCommand,sendDisabled:!k.connected||!k.command.trim(),inputDisabled:!k.connected,commandHistory:k.history,onGraphLinkMessage:Et,onCopyMessage:()=>E(`Copied to clipboard`,`success`),onSendToJsonPath:Dt,onUploadMockData:fe,successfulUploadPaths:de}):(0,z.jsx)(As,{uploadPath:R,onSuccess:me,onClose:pe,onError:he,description:`The JSON becomes the instance's input.body and the optional headers its input.header. In a shared session every member's instance receives them.`,inputPathHints:Ct.inputBodyPaths,inputHeaderHints:Ct.inputHeaderNames},R):(0,z.jsx)(Ss,{mode:ft.action===`edit-node`?`edit`:`create`,formState:ft.formState,phase:ft.phase,lockReason:ft.phase===`sending`?`sending`:ft.connectionLost?`disconnected`:null,serverMessage:ft.serverMessage,validationErrors:ut.validationErrors,onFormStateChange:ut.updateFormState,onSubmit:ut.submit,onClose:ut.close,onOpenSchemaPanel:Bt===null?void 0:()=>Rt(Bt,!0)})}),(0,z.jsx)(te,{className:F.resizeHandle,"aria-label":`Resize panels`})]}),(0,z.jsx)(A,{defaultSize:Nt,minSize:`20%`,children:(0,z.jsx)(rs,{tabs:m,payload:S,onChange:C,validation:w,onFormat:Pt,onUpload:s?k.uploadPayload:void 0,graphData:N,graphName:ot,activeTab:ae,onTabChange:I,onGraphRenderError:e=>E(e,`error`),onGraphDataCopySuccess:()=>E(`Graph JSON copied to clipboard!`,`success`),onGraphDataCopyError:()=>E(`Copy failed`,`error`),graphRunControls:f?{phase:Ct.phase,canInstantiate:Ct.canInstantiate,canUpload:Ct.canUpload,canRun:Ct.canRun,disabledReason:Ct.disabledReason,inputBodyPaths:Ct.inputBodyPaths,inputHeaderNames:Ct.inputHeaderNames,onInstantiate:Ct.instantiateGraph,onUpload:It,onRun:Ct.runGraph}:void 0,isGraphRefreshing:le,onClipNode:c?et:void 0,onClipNodes:c?tt:void 0,onClipboardDrop:c?$e:void 0,onImportFiles:d?ve.importFiles:void 0,onImportGraph:d?ve.openFilePicker:void 0,onDownloadGraph:m.includes(`graph`)?xe:void 0,isConnected:k.connected,supportsAuthoring:d,onCreateNode:d?ut.openCreateNode:void 0,onCreateConnection:d?_t:void 0,onEditNode:d?yt:void 0,onDeleteNode:d?bt:void 0,onDeleteNodes:d?ut.deleteNodes:void 0,onDeleteConnections:d?vt:void 0,panelLayoutKey:`${kt??`closed`}|${Ke}|${Ne}`,helpPanel:l&&Ne?((e,t)=>(0,z.jsx)(ad,{activeTopic:je,contentProfile:u,onNavigate:Me,onClose:()=>Pe(!1),onToggleMaximize:e,isMaximized:t})):void 0})}),c&&Ke&&(0,z.jsxs)(z.Fragment,{children:[(0,z.jsx)(te,{className:F.resizeHandle,"aria-label":`Resize clipboard`}),(0,z.jsx)(A,{defaultSize:Rd(Ld),minSize:`10%`,maxSize:`40%`,children:(0,z.jsx)(ed,{connected:k.connected,onPasteToInput:Qe})})]})]})]})}function Bd(){let e=ue[0].path;return(0,z.jsx)(he,{children:(0,z.jsx)(Nu,{children:(0,z.jsx)(E,{children:(0,z.jsxs)(S,{children:[ue.map(e=>(0,z.jsx)(C,{path:e.path,element:(0,z.jsx)(zd,{config:e},e.path)},e.path)),(0,z.jsx)(C,{path:`*`,element:(0,z.jsx)(T,{to:e,replace:!0})})]})})})})}(0,ie.createRoot)(document.getElementById(`root`)).render((0,z.jsx)(P.StrictMode,{children:(0,z.jsx)(Bd,{})}));
//# sourceMappingURL=index-l_GIA_Pq.js.map