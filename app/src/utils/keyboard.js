/**
 * 键盘适配工具
 * 解决移动端输入法覆盖输入框的问题
 */

let originalViewportHeight = 0;
let keyboardVisible = false;

export function initKeyboardAdapter() {
  // 记录初始视口高度
  originalViewportHeight = window.innerHeight;
  
  // 监听视口大小变化
  window.addEventListener('resize', handleViewportChange);
  
  // 监听页面滚动，在键盘弹出时调整滚动位置
  document.addEventListener('focusin', handleInputFocus);
  document.addEventListener('focusout', handleInputBlur);
}

function handleViewportChange() {
  const currentHeight = window.innerHeight;
  const heightDiff = originalViewportHeight - currentHeight;
  
  // 如果高度差异超过150px，认为是键盘弹出
  if (heightDiff > 150) {
    if (!keyboardVisible) {
      keyboardVisible = true;
      handleKeyboardShow(heightDiff);
    }
  } else {
    if (keyboardVisible) {
      keyboardVisible = false;
      handleKeyboardHide();
    }
  }
}

function handleInputFocus(event) {
  const target = event.target;
  
  // 只处理输入框和文本域
  if (!['INPUT', 'TEXTAREA'].includes(target.tagName)) {
    return;
  }
  
  // 延迟处理，等待键盘完全弹出
  setTimeout(() => {
    scrollToInput(target);
  }, 300);
}

function handleInputBlur() {
  // 输入框失焦时的处理
  setTimeout(() => {
    if (!keyboardVisible) {
      resetScroll();
    }
  }, 100);
}

function handleKeyboardShow(keyboardHeight) {
  // 键盘弹出时的处理
  document.body.style.paddingBottom = `${keyboardHeight}px`;
  document.body.classList.add('keyboard-visible');
}

function handleKeyboardHide() {
  // 键盘隐藏时的处理
  document.body.style.paddingBottom = '';
  document.body.classList.remove('keyboard-visible');
}

function scrollToInput(element) {
  if (!element || !keyboardVisible) return;
  
  const rect = element.getBoundingClientRect();
  const viewportHeight = window.innerHeight;
  
  // 如果输入框被键盘遮挡
  if (rect.bottom > viewportHeight - 50) {
    const scrollOffset = rect.bottom - viewportHeight + 100;
    
    // 平滑滚动到合适位置
    window.scrollBy({
      top: scrollOffset,
      behavior: 'smooth'
    });
  }
}

function resetScroll() {
  // 重置滚动位置（可选）
  // window.scrollTo({ top: 0, behavior: 'smooth' });
}

export function destroyKeyboardAdapter() {
  window.removeEventListener('resize', handleViewportChange);
  document.removeEventListener('focusin', handleInputFocus);
  document.removeEventListener('focusout', handleInputBlur);
  
  // 清理样式
  document.body.style.paddingBottom = '';
  document.body.classList.remove('keyboard-visible');
} 