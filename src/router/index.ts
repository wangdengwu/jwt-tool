import { createRouter, createWebHistory } from 'vue-router'

const routes = [
    {
        path: '/',
        redirect: '/verify'
    },
    {
        path: '/verify',
        name: 'Verify',
        component: () => import('../views/VerifyView.vue')
    },
    {
        path: '/new',
        name: 'New',
        component: () => import('../views/NewView.vue')
    },
    {
        path: '/keys',
        name: 'Keys',
        component: () => import('../views/KeysView.vue')
    }
]

const router = createRouter({
    history: createWebHistory(),
    routes
})

export default router