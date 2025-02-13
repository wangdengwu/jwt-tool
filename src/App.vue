<script setup lang="ts">
import {CirclePlusFilled, QuestionFilled, Tools} from "@element-plus/icons-vue";
</script>

<template>
  <el-container class="layout-container">
    <el-aside class="aside-container">
      <h3 class="mb-2">功能菜单</h3>
      <el-menu
          router
          :default-active="$route.path"
          class="full-height-menu"
      >
        <el-menu-item index="/verify">
          <el-icon>
            <QuestionFilled/>
          </el-icon>
          <span>校验</span>
        </el-menu-item>
        <el-menu-item index="/new">
          <el-icon>
            <CirclePlusFilled/>
          </el-icon>
          <span>生成</span>
        </el-menu-item>
        <el-menu-item index="/keys">
          <el-icon>
            <Tools/>
          </el-icon>
          <span>密钥</span>
        </el-menu-item>
      </el-menu>
    </el-aside>
    <el-main class="main-container">
      <router-view v-slot="{ Component }">
        <keep-alive>
          <component :is="Component"/>
        </keep-alive>
      </router-view>
    </el-main>
  </el-container>
</template>

<style>
body {
  overflow: hidden;
}
</style>

<style scoped>
/* 关键样式 */
.layout-container {
  height: 100vh; /* 确保容器占满视口高度 */
  display: flex;
}

.aside-container {
  /* 侧边栏宽度建议使用固定值 */
  width: 200px !important;
  overflow: hidden;
  /* 继承父容器高度 */
  height: inherit;
}

.main-container {
  /* 主内容区自动填充剩余空间 */
  flex: 1;
  padding: 20px;
  overflow: auto;
}

.full-height-menu {
  /* 重要：让菜单占满侧边栏高度 */
  height: 100%;
}
</style>